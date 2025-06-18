use crate::asl::ast::{AstAnalyzer, AstNode, ExprTApplyData, InstructionSet, NodeData, NodeSubtype, OperandMetadata, StmtTCallData, TypedIdentifier};
use crate::asl::CodegenError;
use crate::unwrap_node_data;
use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::rc::Rc;
use syn::{parse_file, Item, LitInt, LitStr};

#[derive(Debug, Clone)]
struct VariableMetadata {
    type_name: String,
}

#[derive(Debug, Clone)]
struct HelperMetadata {
    is_builtin: bool,
    implicit_param_count: usize,
}

pub struct DecodeTranspiler {
    analyzer: Rc<AstAnalyzer>,

    supported_opcodes: BTreeSet<String>,
    opcodes_and_operands: BTreeMap<String, Vec<OperandMetadata>>,
    enum_types: BTreeSet<String>,
    enum_variants: BTreeMap<String, String>,
    constant_names: BTreeSet<String>,

    decode_blocks: BTreeMap<String, TokenStream>,
    helpers: BTreeMap<String, Option<TokenStream>>,
    helper_table: BTreeMap<String, HelperMetadata>,

    see_mappings: BTreeMap<String, String>,
}

impl DecodeTranspiler {
    pub fn new(analyzer: Rc<AstAnalyzer>, helpers_file_path: &str, supported_opcodes: BTreeSet<String>) -> Result<Self, CodegenError> {
        let base_helpers = fs::read_to_string(helpers_file_path).map_err(|e| CodegenError::IoError(format!("Failed to read helpers file: {}", e)))?;
        let mut helpers = BTreeMap::new();
        let mut helper_table = BTreeMap::new();

        let syntax_tree = parse_file(&*base_helpers).map_err(|e| CodegenError::IoError(format!("Failed to parse helpers file: {}", e)))?;

        for item in syntax_tree.items {
            if let Item::Fn(item_fn) = item {
                let helper_name = item_fn.sig.ident.to_string();
                helpers.insert(helper_name.clone(), None);
                helper_table.insert(
                    helper_name,
                    HelperMetadata {
                        is_builtin: true,
                        implicit_param_count: 0,
                    },
                );
            }
        }

        let opcodes_and_operands = analyzer.get_opcode_and_operands(InstructionSet::A64)?.clone();
        let (enum_types_ref, enum_variants_ref) = analyzer.get_enum_types_and_variants()?;
        let enum_types = enum_types_ref.clone();
        let enum_variants = enum_variants_ref.clone();
        let constant_names = analyzer.get_constant_names()?;

        let see_mappings = [
            ("PACIA", "aarch64_integer_pac_pacia_dp_1src"),
            ("PACIB", "aarch64_integer_pac_pacib_dp_1src"),
            ("AUTIA", "aarch64_integer_pac_autia_dp_1src"),
            ("AUTIB", "aarch64_integer_pac_autib_dp_1src"),
            ("XPACLRI", "aarch64_integer_pac_strip_dp_1src"),
            ("HINT", "aarch64_system_hints"),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<BTreeMap<String, String>>();

        Ok(Self {
            analyzer,

            supported_opcodes,
            opcodes_and_operands,
            enum_types,
            enum_variants,
            constant_names,

            decode_blocks: BTreeMap::new(),
            helpers,
            helper_table,

            see_mappings,
        })
    }

    pub fn translate_enums(&mut self) -> Result<TokenStream, CodegenError> {
        let analyzer = Rc::clone(&self.analyzer);
        let mut translated_enums = TokenStream::new();

        for (enum_type, enum_variants) in analyzer.get_enums()? {
            let enum_type_ident = Ident::new(enum_type, Span::call_site());
            let enum_variant_idents = enum_variants
                .iter()
                .map(|variant| Ident::new(variant, Span::call_site()))
                .collect::<Vec<Ident>>();
            let default_variant_ident = enum_variant_idents
                .first()
                .ok_or(CodegenError::EnumWithEmptyVariants(enum_type.to_string()))?;

            translated_enums.extend(quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCount)]
                #[repr(u8)]
                pub enum #enum_type_ident {
                    #(#enum_variant_idents,)*
                }
                impl Default for #enum_type_ident {
                    fn default() -> Self {
                        #enum_type_ident::#default_variant_ident
                    }
                }
            });
        }

        Ok(translated_enums)
    }

    pub fn translate_constants(&mut self) -> Result<TokenStream, CodegenError> {
        let analyzer = Rc::clone(&self.analyzer);
        let mut translated_constants = TokenStream::new();

        for (name, (ty, expr)) in analyzer.get_constants()? {
            let ty_tokens = self.translate_type(ty)?;
            let name_ident = Ident::new(name, Span::call_site());
            let expr_tokens = self.translate_expression(expr, &BTreeMap::new(), false)?;

            translated_constants
                .extend(quote! { pub static #name_ident: Lazy<Result<#ty_tokens, AArch64LifterError>> = Lazy::new(|| { Ok(#expr_tokens) }); })
        }

        Ok(translated_constants)
    }

    pub fn translate_decode_logic(&mut self) -> Result<TokenStream, CodegenError> {
        let analyzer = Rc::clone(&self.analyzer);
        let decoder_case = analyzer.get_decoder_case(InstructionSet::A64)?;

        self.translate_decoder_case(decoder_case)
    }

    pub fn get_decode_blocks(&self) -> &BTreeMap<String, TokenStream> {
        &self.decode_blocks
    }

    pub fn get_helpers(&self) -> Vec<TokenStream> {
        self.helpers
            .values()
            .filter_map(|helper_opt| helper_opt.as_ref())
            .cloned()
            .collect::<Vec<TokenStream>>()
    }

    fn translate_decoder_case(&mut self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::DecoderCase);

        let slices = data
            .decode_slices
            .iter()
            .map(|slice| self.translate_decoder_slice(slice))
            .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

        let length = data.decode_alts.len();
        let mut alts = data
            .decode_alts
            .iter()
            .rev()
            .enumerate()
            .map(|(i, alt)| self.translate_decoder_alt(alt, &slices, i == 0, i == length - 1))
            .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

        if let Some(last_alt) = alts.last() {
            let mut tokens = last_alt.clone().into_iter();
            let is_else_if = match (tokens.next(), tokens.next()) {
                (Some(TokenTree::Ident(ref ident1)), Some(TokenTree::Ident(ref ident2))) => ident1 == "else" && ident2 == "if",
                _ => false,
            };

            if is_else_if {
                alts.push(quote! {
                    else {
                        return Ok(common::types::Instruction::UNDEF);
                    }
                });
            }
        }

        Ok(quote! { #(#alts)* })
    }

    fn translate_decoder_slice(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Slice => {
                let data = unwrap_node_data!(node, NodeData::DecoderSlice);
                let offset = data.int1 as usize;
                let length = data.int2 as usize;

                Ok(quote! { reader.extract_slice(#offset, #length)? })
            }
            NodeSubtype::FieldName => {
                let data = unwrap_node_data!(node, NodeData::DecoderSliceFieldName);
                let field_name_slice = Ident::new(&(data.ident.clone() + "_slice"), Span::call_site());

                Ok(quote! { #field_name_slice })
            }
            NodeSubtype::Concat => {
                let data = unwrap_node_data!(node, NodeData::DecoderSliceConcat);
                let field_names = data
                    .idents
                    .iter()
                    .map(|ident| Ident::new(ident, Span::call_site()))
                    .collect::<Vec<Ident>>();
                let fields = field_names
                    .iter()
                    .map(|field_name| quote! { &#field_name[..] })
                    .collect::<Vec<TokenStream>>();

                Ok(quote! { [#(#fields),*].concat() })
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_decoder_alt(&mut self, node: &AstNode, slices: &Vec<TokenStream>, first: bool, last: bool) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::DecoderAlt);

        let patterns = data
            .decode_patterns
            .iter()
            .enumerate()
            .map(|(i, pattern)| self.translate_decoder_pattern(pattern, &slices[i]))
            .collect::<Result<Vec<Option<TokenStream>>, CodegenError>>()?
            .into_iter()
            .filter_map(|p| p)
            .collect::<Vec<TokenStream>>();

        let body = self.translate_decoder_body(&data.decode_body)?;

        if data.decode_patterns.is_empty() {
            Ok(quote! { #body })
        } else if patterns.is_empty() {
            if !last {
                return Err(CodegenError::InvalidDecodeCaseOrder);
            }

            Ok(quote! {
                else {
                    #body
                }
            })
        } else {
            let cond_prefix = if first {
                quote! { if }
            } else {
                quote! { else if }
            };

            Ok(quote! {
                #cond_prefix #(#patterns)&&* {
                    #body
                }
            })
        }
    }

    fn translate_decoder_pattern(&self, node: &AstNode, slice: &TokenStream) -> Result<Option<TokenStream>, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Bits => {
                let data = unwrap_node_data!(node, NodeData::DecoderPatternBits);
                let lit_str = LitStr::new(&data.bits_lit, Span::call_site());

                Ok(Some(quote! { decode::BitReader::match_bits(&#slice, #lit_str) }))
            }
            NodeSubtype::Mask => {
                let data = unwrap_node_data!(node, NodeData::DecoderPatternMask);
                let lit_str = LitStr::new(&data.mask_lit, Span::call_site());

                Ok(Some(quote! { decode::BitReader::match_bits(&#slice, #lit_str) }))
            }
            NodeSubtype::Wildcard => Ok(None),
            NodeSubtype::Not => {
                let data = unwrap_node_data!(node, NodeData::DecoderPatternNot);
                let pattern = self
                    .translate_decoder_pattern(&data.decode_pattern, slice)?
                    .ok_or(CodegenError::InvalidDecodePattern)?;

                Ok(Some(quote! { !#pattern }))
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_decoder_body(&mut self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::UNPRED => Ok(quote! { instruction = common::types::Instruction::UNPRED; }),
            NodeSubtype::UNALLOC => Ok(quote! { instruction = common::types::Instruction::UNALLOC; }),
            NodeSubtype::NOP => Ok(quote! { instruction = common::types::Instruction::NOP; }),
            NodeSubtype::Encoding => {
                let encoding_data = unwrap_node_data!(node, NodeData::DecoderBodyEncoding);
                let encoding_name = &encoding_data.ident;

                let analyzer = Rc::clone(&self.analyzer);
                let instruction_node = analyzer.get_instruction_def_by_encoding_name(encoding_name)?;
                let instruction_data = unwrap_node_data!(instruction_node, NodeData::DeclInstructionDefn);
                let instruction_name = &instruction_data.ident;

                let encoding_node = instruction_data
                    .encodings
                    .iter()
                    .find(|encoding_block| {
                        let encoding_data = unwrap_node_data!(encoding_block, NodeData::EncodingBlock);
                        encoding_data.ident1 == encoding_name.as_str()
                    })
                    .unwrap();
                let supported = self.supported_opcodes.contains(instruction_name);
                let decode_logic = self.translate_encoding_and_postdecode_blocks(encoding_node, &instruction_data.opt_stmts, instruction_name, supported)?;

                // let opcode_ident = Ident::new(instruction_name, Span::call_site());
                // let log_lit_str = LitStr::new(
                //     &format!("Decoding instruction {} encoding {}", opcode_ident, encoding_name),
                //     Span::call_site(),
                // );

                Ok(quote! {
                    // println!(#log_lit_str);
                    #decode_logic
                })
            }
            NodeSubtype::Decoder => {
                let data = unwrap_node_data!(node, NodeData::DecoderBodyDecoder);

                let instr_fields = data
                    .instr_fields
                    .iter()
                    .map(|instr_field| self.translate_instruction_field_for_decoder_body(instr_field))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                let decode_case = self.translate_decoder_case(&data.decode_case)?;

                Ok(quote! {
                    #(#instr_fields)*
                    #decode_case
                })
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_encoding_and_postdecode_blocks(
        &mut self,
        node: &AstNode,
        opt_postdecode_stmts: &Option<Vec<AstNode>>,
        instruction_name: &str,
        supported: bool,
    ) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::EncodingBlock);

        let instr_fields = data
            .instr_fields
            .iter()
            .map(|instr_field| self.translate_instruction_field_for_decode_blocks(instr_field))
            .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

        let encoding_name = &data.ident1;
        let decode_function_name = Ident::new(&format!("decode_{}", encoding_name), Span::call_site());

        if self.decode_blocks.get(encoding_name).is_none() {
            let mut symbol_table = data
                .instr_fields
                .iter()
                .map(|instr_field| {
                    let data = unwrap_node_data!(instr_field, NodeData::IField);
                    let metadata = VariableMetadata {
                        type_name: "bits".to_string(),
                    };
                    Ok((data.ident.clone(), metadata))
                })
                .collect::<Result<BTreeMap<String, VariableMetadata>, CodegenError>>()?;

            let mut decode_statements = Vec::new();

            for stmt in data
                .stmts
                .iter()
                .chain(opt_postdecode_stmts.as_ref().map(|v| v.iter()).into_iter().flatten())
            {
                if supported {
                    match self.translate_statement(stmt, &mut symbol_table) {
                        Ok(tokens) => decode_statements.push(tokens),
                        Err(CodegenError::NotImplemented(file, line)) => {
                            let line = line as usize;
                            decode_statements.push(quote! {
                                return Err(AArch64LifterError::UnimplementedInstruction(#file.to_string(), #line));
                            });
                        }
                        Err(CodegenError::MissingBuiltinFunctionImplementation(function_name)) => {
                            decode_statements.push(quote! {
                                return Err(AArch64LifterError::UnimplementedBuiltinFunction(#function_name.to_string()));
                            });
                        }
                        Err(e) => return Err(e),
                    }
                } else {
                    decode_statements.push(self.translate_statement_unsupported(stmt, &mut symbol_table)?);
                }
            }

            let opcode_ident = Ident::new(instruction_name, Span::call_site());
            let operands_struct_ident = Ident::new(format!("{}_operands", instruction_name).as_str(), Span::call_site());
            let operands = self
                .opcodes_and_operands
                .get(instruction_name)
                .ok_or(CodegenError::MissingOperandAnalysisForInstruction(instruction_name.to_string()))?;
            let operand_field_idents = operands
                .iter()
                .map(|operand| Ident::new(&operand.name, Span::call_site()))
                .collect::<Vec<Ident>>();

            self.decode_blocks.insert(
                encoding_name.to_string(),
                quote! {
                    pub fn #decode_function_name(reader: &mut decode::BitReader) -> Result<common::types::Instruction, AArch64LifterError> {
                        #(#instr_fields)*
                        #(#decode_statements)*
                        Ok(common::types::Instruction::#opcode_ident(Box::new(common::types::#operands_struct_ident {
                            #(#operand_field_idents),*
                        })))
                    }
                },
            );
        }

        Ok(quote! {
            match decode::generated::decode_blocks::#decode_function_name(reader) {
                Ok(instr) => instruction = instr,
                Err(AArch64LifterError::UndefinedInstruction) => instruction = common::types::Instruction::UNDEF,
                Err(e) => return Err(e),
            }
        })
    }

    fn translate_instruction_field_for_decoder_body(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::IField);

        let field_name_slice = Ident::new(&format!("{}_slice", &data.ident), Span::call_site());
        let offset = data.int1 as usize;
        let length = data.int2 as usize;

        Ok(quote! {
            let #field_name_slice = reader.extract_slice(#offset, #length)?;
        })
    }

    fn translate_instruction_field_for_decode_blocks(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::IField);

        let field_name = Ident::new(&data.ident, Span::call_site());
        let offset = data.int1 as usize;
        let length = data.int2 as usize;

        Ok(quote! {
            let #field_name = common::types::bits::from_bits_literal(&reader.extract_slice(#offset, #length)?)?;
        })
    }

    fn translate_statement(&mut self, node: &AstNode, symbol_table: &mut BTreeMap<String, VariableMetadata>) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::VarDeclsNoInit => {
                let data = unwrap_node_data!(node, NodeData::StmtVarDeclsNoInit);

                let ty = self.translate_type(&data.ty)?;
                let name_idents = data
                    .idents
                    .iter()
                    .map(|ident| Ident::new(&ident, Span::call_site()))
                    .collect::<Vec<Ident>>();

                let type_name = self.get_type_name(&data.ty)?;
                for ident in data.idents.iter() {
                    symbol_table.insert(
                        ident.to_string(),
                        VariableMetadata {
                            type_name: type_name.clone(),
                        },
                    );
                }

                let default_init = self.translate_type_to_default_init(&data.ty, symbol_table, &ty)?;

                if name_idents.len() == 1 {
                    let name_ident = &name_idents[0];

                    Ok(quote! { let mut #name_ident: #ty = #default_init; })
                } else {
                    let tys = vec![ty.clone(); name_idents.len()];
                    let default_inits = vec![default_init.clone(); name_idents.len()];

                    Ok(quote! { let (#(mut #name_idents),*): (#(#tys),*) = (#(#default_inits),*); })
                }
            }
            NodeSubtype::VarDecl => {
                let data = unwrap_node_data!(node, NodeData::StmtVarDecl);

                let ty = self.translate_type(&data.ty)?;
                let name_ident = Ident::new(&data.ident, Span::call_site());

                symbol_table.insert(
                    data.ident.clone(),
                    VariableMetadata {
                        type_name: self.get_type_name(&data.ty)?,
                    },
                );

                let value = self.translate_expression(&data.expr, symbol_table, true)?;

                Ok(quote! {
                    let mut #name_ident: #ty = #value;
                })
            }
            NodeSubtype::ConstDecl => {
                let data = unwrap_node_data!(node, NodeData::StmtConstDecl);

                let ty = self.translate_type(&data.ty)?;
                let name_ident = Ident::new(&data.ident, Span::call_site());

                symbol_table.insert(
                    data.ident.clone(),
                    VariableMetadata {
                        type_name: self.get_type_name(&data.ty)?,
                    },
                );

                let value = self.translate_expression(&data.expr, symbol_table, true)?;

                Ok(quote! {
                    let #name_ident: #ty = #value;
                })
            }
            NodeSubtype::Assign => {
                let data = unwrap_node_data!(node, NodeData::StmtAssign);

                let name_ident = self.translate_left_expression(&data.l_expr)?;

                let expression = self.translate_expression(&data.expr, symbol_table, true)?;

                Ok(quote! {
                    #name_ident = #expression;
                })
            }
            NodeSubtype::FunReturn => {
                let data = unwrap_node_data!(node, NodeData::StmtFunReturn);

                let value = self.translate_expression(&data.expr, symbol_table, false)?;

                Ok(quote! { return Ok(#value); })
            }
            NodeSubtype::ProcReturn => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Assert => {
                let data = unwrap_node_data!(node, NodeData::StmtAssert);

                let expression = self.translate_expression(&data.expr, symbol_table, false)?;

                Ok(quote! { assert_eq!(#expression, common::types::boolean::TRUE); })
            }
            NodeSubtype::Unpred => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ConstrainedUnpred => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ImpDef => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ExceptionTaken => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::DepUnpred => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::DepImpDef => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::DepUndefined => Ok(quote! {
                return Err(AArch64LifterError::UndefinedInstruction);
            }),
            NodeSubtype::See => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Throw => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::DecodeExecute => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::TCall => {
                let data = unwrap_node_data!(node, NodeData::StmtTCall);

                self.translate_t_callable(data)?;

                let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);
                let procedure_name = Ident::new(&normalized_ident, Span::call_site());

                match self.helper_table.get(&normalized_ident) {
                    Some(metadata) => {
                        if !metadata.is_builtin && metadata.implicit_param_count != data.exprs1.len() {
                            return Err(CodegenError::WrongImplicitParamCount(
                                normalized_ident,
                                data.exprs1.len(),
                                metadata.implicit_param_count,
                            ));
                        }
                    }
                    None => {}
                }

                let mut arguments = Vec::<TokenStream>::new();
                for argument_node in data.exprs2.iter().chain(data.exprs1.iter()) {
                    let argument = self.translate_expression(argument_node, symbol_table, true)?;
                    arguments.push(argument);
                }

                Ok(quote! { decode::helpers::#procedure_name(#(#arguments),*)?; })
            }
            NodeSubtype::If => {
                let data = unwrap_node_data!(node, NodeData::StmtIf);

                let if_condition = self.translate_expression(&data.expr, symbol_table, false)?;
                let mut if_block_symbol_table = symbol_table.clone();
                let if_statements = data
                    .stmts1
                    .iter()
                    .map(|statement| self.translate_statement(statement, &mut if_block_symbol_table))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;
                let s_elsifs = data
                    .s_elsifs
                    .iter()
                    .map(|elsif| self.translate_s_elsif(elsif, symbol_table))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;
                let mut else_block_symbol_table = symbol_table.clone();
                let else_statements = data
                    .stmts2
                    .iter()
                    .map(|statement| self.translate_statement(statement, &mut else_block_symbol_table))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                Ok(quote! {
                    if (#if_condition) == common::types::boolean::TRUE {
                        #(#if_statements)*
                    }
                    #(#s_elsifs)*
                    else {
                        #(#else_statements)*
                    }
                })
            }
            NodeSubtype::Case => {
                let data = unwrap_node_data!(node, NodeData::StmtCase);

                let expression = self.translate_expression(&data.expr, symbol_table, false)?;
                let alts = data
                    .alts
                    .iter()
                    .enumerate()
                    .map(|(i, alt)| self.translate_alt(alt, &expression, i == 0, symbol_table))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                if let Some(stmts) = &data.opt_stmts {
                    let mut default_block_symbol_table = symbol_table.clone();
                    let default_statements = stmts
                        .iter()
                        .map(|statement| self.translate_statement(statement, &mut default_block_symbol_table))
                        .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                    Ok(quote! {
                        #(#alts)*
                        else {
                            #(#default_statements)*
                        }
                    })
                } else {
                    Ok(quote! {
                        #(#alts)*
                    })
                }
            }
            NodeSubtype::For => {
                let data = unwrap_node_data!(node, NodeData::StmtFor);

                let loop_counter = Ident::new(&data.ident, Span::call_site());
                let initial_value = self.translate_expression(&data.expr1, symbol_table, true)?;
                let final_value = self.translate_expression(&data.expr2, symbol_table, true)?;
                let (loop_condition, loop_update) = match data.direction.node_subtype {
                    NodeSubtype::Up => (
                        quote! { #loop_counter <= #final_value },
                        quote! { #loop_counter = #loop_counter + common::types::integer::one(); },
                    ),
                    NodeSubtype::Down => (
                        quote! { #loop_counter >= #final_value },
                        quote! { #loop_counter = #loop_counter - common::types::integer::one(); },
                    ),
                    _ => return Err(CodegenError::InvalidNodeType(data.direction.node_type, data.direction.node_subtype)),
                };

                let mut body_block_symbol_table = symbol_table.clone();
                let statements = data
                    .stmts
                    .iter()
                    .map(|statement| self.translate_statement(statement, &mut body_block_symbol_table))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                Ok(quote! {
                    let mut #loop_counter = #initial_value;
                    let start = #initial_value;
                    let end = #final_value;
                    while #loop_condition {
                        #(#statements)*
                        #loop_update
                    }
                })
            }
            NodeSubtype::While => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Repeat => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Try => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_statement_unsupported(&mut self, node: &AstNode, symbol_table: &mut BTreeMap<String, VariableMetadata>) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::VarDeclsNoInit => {
                let data = unwrap_node_data!(node, NodeData::StmtVarDeclsNoInit);

                let ty = self.translate_type(&data.ty)?;
                let name_idents = data
                    .idents
                    .iter()
                    .filter(|ident| ["d", "n", "dn", "s", "t", "t2"].contains(&ident.as_str()))
                    .map(|ident| Ident::new(&ident, Span::call_site()))
                    .collect::<Vec<Ident>>();

                let type_name = self.get_type_name(&data.ty)?;
                for ident in data.idents.iter() {
                    symbol_table.insert(
                        ident.to_string(),
                        VariableMetadata {
                            type_name: type_name.clone(),
                        },
                    );
                }

                let default_init = self.translate_type_to_default_init(&data.ty, symbol_table, &ty)?;

                if name_idents.is_empty() {
                    Ok(quote! {})
                } else if name_idents.len() == 1 {
                    let name_ident = &name_idents[0];

                    Ok(quote! { let mut #name_ident: #ty = #default_init; })
                } else {
                    let tys = vec![ty.clone(); name_idents.len()];
                    let default_inits = vec![default_init.clone(); name_idents.len()];

                    Ok(quote! { let (#(mut #name_idents),*): (#(#tys),*) = (#(#default_inits),*); })
                }
            }
            NodeSubtype::VarDecl => {
                let data = unwrap_node_data!(node, NodeData::StmtVarDecl);

                if !["d", "n", "dn", "s", "t", "t2"].contains(&data.ident.as_str()) {
                    return Ok(quote! {});
                }

                let ty = self.translate_type(&data.ty)?;
                let name_ident = Ident::new(&data.ident, Span::call_site());

                symbol_table.insert(
                    data.ident.clone(),
                    VariableMetadata {
                        type_name: self.get_type_name(&data.ty)?,
                    },
                );

                let value = self.translate_expression(&data.expr, symbol_table, true)?;

                Ok(quote! {
                    let mut #name_ident: #ty = #value;
                })
            }
            NodeSubtype::Assign => {
                let data = unwrap_node_data!(node, NodeData::StmtAssign);

                match self.translate_left_expression_unsupported(&data.l_expr)? {
                    Some(name_ident) => {
                        let expression = self.translate_expression(&data.expr, symbol_table, true)?;

                        Ok(quote! {
                            #name_ident = #expression;
                        })
                    }
                    None => Ok(quote! {}),
                }
            }
            NodeSubtype::Case => {
                let data = unwrap_node_data!(node, NodeData::StmtCase);

                let mut whole_is_empty = true;
                for alt_node in data.alts.iter() {
                    let alt = unwrap_node_data!(alt_node, NodeData::Alt);

                    let mut alt_block_symbol_table = symbol_table.clone();
                    let branch_is_empty = alt
                        .stmts
                        .iter()
                        .map(|statement| self.translate_statement_unsupported(statement, &mut alt_block_symbol_table))
                        .collect::<Result<Vec<TokenStream>, CodegenError>>()?
                        .iter()
                        .any(|ts| ts.is_empty());

                    if !branch_is_empty {
                        whole_is_empty = false;
                        break;
                    }
                }
                if let Some(stmts) = &data.opt_stmts {
                    let mut default_block_symbol_table = symbol_table.clone();
                    let branch_is_empty = stmts
                        .iter()
                        .map(|statement| self.translate_statement_unsupported(statement, &mut default_block_symbol_table))
                        .collect::<Result<Vec<TokenStream>, CodegenError>>()?
                        .iter()
                        .any(|ts| ts.is_empty());

                    if !branch_is_empty {
                        whole_is_empty = false;
                    }
                }
                if whole_is_empty {
                    return Ok(quote! {});
                }

                let expression = self.translate_expression(&data.expr, symbol_table, false)?;
                let alts = data
                    .alts
                    .iter()
                    .enumerate()
                    .map(|(i, alt)| self.translate_alt_unsupported(alt, &expression, i == 0, symbol_table))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                if let Some(stmts) = &data.opt_stmts {
                    let mut default_block_symbol_table = symbol_table.clone();
                    let default_statements = stmts
                        .iter()
                        .map(|statement| self.translate_statement_unsupported(statement, &mut default_block_symbol_table))
                        .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                    Ok(quote! {
                        #(#alts)*
                        else {
                            #(#default_statements)*
                        }
                    })
                } else {
                    Ok(quote! {
                        #(#alts)*
                    })
                }
            }
            NodeSubtype::See => {
                let data = unwrap_node_data!(node, NodeData::StmtSee);

                match data.expr.node_subtype {
                    NodeSubtype::LitString => {
                        let data = unwrap_node_data!(data.expr, NodeData::ExprLitString);

                        match self.see_mappings.get(&data.string_lit) {
                            Some(opcode) => {
                                let decode_block_function_name = Ident::new(&format!("decode_{}", opcode), Span::call_site());
                                Ok(quote! {
                                    return Ok(decode::generated::decode_blocks::#decode_block_function_name(reader)?);
                                })
                            }
                            None => Ok(quote! {})
                        }
                    }
                    _ => Err(CodegenError::NotImplemented(file!(), line!())),
                }
            }
            _ => Ok(quote! {})
        }
    }

    fn translate_expression(
        &mut self,
        node: &AstNode,
        symbol_table: &BTreeMap<String, VariableMetadata>,
        needs_clone: bool,
    ) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::If => {
                let data = unwrap_node_data!(node, NodeData::ExprIf);

                let if_condition = self.translate_expression(&data.if_cond_expr, symbol_table, false)?;
                let if_body = self.translate_expression(&data.if_body_expr, symbol_table, needs_clone)?;
                let e_elsifs = data
                    .e_elsifs
                    .iter()
                    .map(|elsif| self.translate_e_elsif(elsif, symbol_table, needs_clone))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;
                let else_body = self.translate_expression(&data.else_body_expr, symbol_table, needs_clone)?;

                Ok(quote! {
                    if (#if_condition) == common::types::boolean::TRUE {
                        #if_body
                    }
                    #(#e_elsifs)*
                    else {
                        #else_body
                    }
                })
            }
            NodeSubtype::Binop => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Unop => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Field => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Fields => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Slices => {
                let data = unwrap_node_data!(node, NodeData::ExprSlices);

                let mut expression = self.translate_expression(&data.expr, symbol_table, false)?;

                for slice in data.slices.iter() {
                    expression = self.translate_slice(slice, &expression, symbol_table)?;
                }

                Ok(quote! { #expression })
            }
            NodeSubtype::In => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Var => {
                let data = unwrap_node_data!(node, NodeData::ExprVar);

                let name = &data.ident;
                let name_ident = Ident::new(&data.ident, Span::call_site());

                if let Some(enum_type) = self.enum_variants.get(name) {
                    let enum_ident = Ident::new(enum_type, Span::call_site());
                    Ok(quote! { common::types::#enum_ident::#name_ident })
                } else if self.constant_names.contains(name) {
                    Ok(quote! { common::types::#name_ident.clone()? })
                } else if needs_clone
                    && symbol_table
                        .get(name)
                        .ok_or(CodegenError::MissingVariableInSymbolTable(name.to_string()))?
                        .type_name
                        == "integer"
                {
                    Ok(quote! { #name_ident.clone() })
                } else {
                    Ok(quote! { #name_ident })
                }
            }
            NodeSubtype::Parens => {
                let data = unwrap_node_data!(node, NodeData::ExprParens);

                let expression = self.translate_expression(&data.expr, symbol_table, needs_clone)?;

                Ok(quote! { (#expression) })
            }
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(node, NodeData::ExprTuple);

                let expressions = data
                    .exprs
                    .iter()
                    .map(|expression| self.translate_expression(expression, symbol_table, needs_clone))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                Ok(quote! { (#(#expressions),*) })
            }
            NodeSubtype::Unknown => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ImpDef => {
                let data = unwrap_node_data!(node, NodeData::ExprImpDef);

                let global_variable = self.translate_type_to_implementation_defined(&data.ty)?;

                Ok(quote! { #global_variable })
            }
            NodeSubtype::TApply => {
                let data = unwrap_node_data!(node, NodeData::ExprTApply);

                self.translate_t_applicable(data)?;

                let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);
                let function_name = Ident::new(&normalized_ident, Span::call_site());

                match self.helper_table.get(&normalized_ident) {
                    Some(metadata) => {
                        if !metadata.is_builtin && metadata.implicit_param_count != data.exprs1.len() {
                            return Err(CodegenError::WrongImplicitParamCount(
                                normalized_ident,
                                data.exprs1.len(),
                                metadata.implicit_param_count,
                            ));
                        }
                    }
                    None => {}
                }

                let mut arguments = Vec::<TokenStream>::new();
                for argument_node in data.exprs2.iter().chain(data.exprs1.iter()) {
                    let argument = self.translate_expression(argument_node, symbol_table, true)?;
                    arguments.push(argument);
                }

                Ok(quote! { decode::helpers::#function_name(#(#arguments),*)? })
            }
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitInt => {
                let data = unwrap_node_data!(node, NodeData::ExprLitInt);

                assert!(!data.int_lit.starts_with('-'));

                let lit_int = LitInt::new(&data.int_lit, Span::call_site());

                Ok(quote! { common::types::integer::from(#lit_int) })
            }
            NodeSubtype::LitHex => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitReal => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitBits => {
                let data = unwrap_node_data!(node, NodeData::ExprLitBits);

                let lit_str = LitStr::new(&data.bits_lit, Span::call_site());

                Ok(quote! { common::types::bits::from_bits_literal(#lit_str)? })
            }
            NodeSubtype::LitMask => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitString => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_left_expression(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Wildcard => Ok(quote! { _ }),
            NodeSubtype::Var => {
                let data = unwrap_node_data!(node, NodeData::LExprVar);

                let name_ident = Ident::new(&data.ident, Span::call_site());

                Ok(quote! { #name_ident })
            }
            NodeSubtype::Field => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Fields => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Slices => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::BitTuple => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(node, NodeData::LExprTuple);

                let contents = data
                    .l_exprs
                    .iter()
                    .map(|expression| self.translate_left_expression(expression))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                Ok(quote! { (#(#contents),*) })
            }
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Write => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ReadWrite => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_left_expression_unsupported(&self, node: &AstNode) -> Result<Option<TokenStream>, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Var => {
                let data = unwrap_node_data!(node, NodeData::LExprVar);

                if !["d", "n", "dn", "s", "t", "t2"].contains(&data.ident.as_str()) {
                    return Ok(None);
                }

                let name_ident = Ident::new(&data.ident, Span::call_site());

                Ok(Some(quote! { #name_ident }))
            }
            _ => Ok(None),
        }
    }

    fn translate_type(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => {
                let data = unwrap_node_data!(node, NodeData::TypeConstructor);

                if data.ident == "integer" || self.enum_types.contains(&data.ident) {
                    let name_ident = Ident::new(&data.ident, Span::call_site());

                    Ok(quote! { common::types::#name_ident })
                } else {
                    Err(CodegenError::UnhandledType(data.ident.to_string()))
                }
            }
            NodeSubtype::Bits => Ok(quote! { common::types::bits }),
            NodeSubtype::App => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::OfExpr => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Register => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(node, NodeData::TypeTuple);

                let tys = data
                    .tys
                    .iter()
                    .map(|ty| self.translate_type(ty))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                Ok(quote! { (#(#tys),*) })
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_type_to_default_init(
        &mut self,
        node: &AstNode,
        symbol_table: &BTreeMap<String, VariableMetadata>,
        ty_ident: &TokenStream,
    ) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => Ok(quote! { #ty_ident::default() }),
            NodeSubtype::Bits => {
                let data = unwrap_node_data!(node, NodeData::TypeBits);

                let bit_length = self.translate_expression(&data.expr, symbol_table, false)?;

                Ok(quote! { common::types::bits::new(0, integer_to_usize!(#bit_length)) })
            }
            NodeSubtype::App => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::OfExpr => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Register => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_type_to_implementation_defined(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => {
                let data = unwrap_node_data!(node, NodeData::TypeConstructor);
                let var_ident = Ident::new(&format!("IMPLEMENTATION_DEFINED_{}", &data.ident), Span::call_site());

                Ok(quote! { common::types::#var_ident })
            }
            NodeSubtype::Bits => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::App => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::OfExpr => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Register => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn get_type_name(&self, node: &AstNode) -> Result<String, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => {
                let data = unwrap_node_data!(node, NodeData::TypeConstructor);
                Ok(data.ident.clone())
            }
            NodeSubtype::Bits => Ok("bits".to_string()),
            NodeSubtype::App => Ok("app".to_string()),
            NodeSubtype::OfExpr => Ok("of".to_string()),
            NodeSubtype::Register => Ok("register".to_string()),
            NodeSubtype::Array => Ok("array".to_string()),
            NodeSubtype::Tuple => Ok("tuple".to_string()),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_s_elsif(&mut self, node: &AstNode, symbol_table: &mut BTreeMap<String, VariableMetadata>) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::SElsifCond);

        let condition = self.translate_expression(&data.expr, symbol_table, false)?;
        let mut elsif_block_symbol_table = symbol_table.clone();
        let statements = data
            .stmts
            .iter()
            .map(|statement| self.translate_statement(&statement, &mut elsif_block_symbol_table))
            .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

        Ok(quote! {
            else if (#condition) == common::types::boolean::TRUE {
                #(#statements)*
            }
        })
    }

    fn translate_e_elsif(
        &mut self,
        node: &AstNode,
        symbol_table: &BTreeMap<String, VariableMetadata>,
        needs_clone: bool,
    ) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::EElsifCond);

        let condition = self.translate_expression(&data.expr1, symbol_table, false)?;
        let expression = self.translate_expression(&data.expr2, symbol_table, needs_clone)?;

        Ok(quote! {
            else if (#condition) == common::types::boolean::TRUE {
                #expression
            }
        })
    }

    fn translate_alt(
        &mut self,
        node: &AstNode,
        expression: &TokenStream,
        first: bool,
        symbol_table: &mut BTreeMap<String, VariableMetadata>,
    ) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::Alt);

        let mut patterns = Vec::new();
        for pattern_node in data.patterns.iter() {
            let pattern = self.translate_pattern(pattern_node)?;
            patterns.push(quote! { #expression.match_with_pattern(#pattern) });
        }
        let guard = match &data.opt_expr {
            Some(expr) => self.translate_expression(expr, symbol_table, false)?,
            None => quote! {true},
        };

        let mut alt_block_symbol_table = symbol_table.clone();
        let statements = data
            .stmts
            .iter()
            .map(|statement| self.translate_statement(&statement, &mut alt_block_symbol_table))
            .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

        let cond_prefix = if first {
            quote! { if }
        } else {
            quote! { else if }
        };

        Ok(quote! {
            #cond_prefix (#(#patterns)||*) && #guard {
                #(#statements)*
            }
        })
    }

    fn translate_alt_unsupported(
        &mut self,
        node: &AstNode,
        expression: &TokenStream,
        first: bool,
        symbol_table: &mut BTreeMap<String, VariableMetadata>,
    ) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::Alt);

        let mut patterns = Vec::new();
        for pattern_node in data.patterns.iter() {
            let pattern = self.translate_pattern(pattern_node)?;
            patterns.push(quote! { #expression.match_with_pattern(#pattern) });
        }
        let guard = match &data.opt_expr {
            Some(expr) => self.translate_expression(expr, symbol_table, false)?,
            None => quote! {true},
        };

        let mut alt_block_symbol_table = symbol_table.clone();
        let statements = data
            .stmts
            .iter()
            .map(|statement| self.translate_statement_unsupported(&statement, &mut alt_block_symbol_table))
            .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

        let cond_prefix = if first {
            quote! { if }
        } else {
            quote! { else if }
        };

        Ok(quote! {
            #cond_prefix (#(#patterns)||*) && #guard {
                #(#statements)*
            }
        })
    }

    fn translate_pattern(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::LitInt => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitHex => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitBits => {
                let data = unwrap_node_data!(node, NodeData::PatLitBits);
                let lit_str = LitStr::new(&data.bits_lit, Span::call_site());

                Ok(quote! { #lit_str })
            }
            NodeSubtype::LitMask => {
                let data = unwrap_node_data!(node, NodeData::PatLitMask);
                let lit_str = LitStr::new(&data.mask_lit, Span::call_site());

                Ok(quote! { #lit_str })
            }
            NodeSubtype::Const => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Wildcard => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Set => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Range => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Single => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_slice(
        &mut self,
        node: &AstNode,
        expression: &TokenStream,
        symbol_table: &BTreeMap<String, VariableMetadata>,
    ) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Single => {
                let data = unwrap_node_data!(node, NodeData::SliceSingle);

                let index = self.translate_expression(&data.expr, symbol_table, false)?;

                Ok(quote! { #expression.extract_slice(integer_to_usize!(#index), 1)? })
            }
            NodeSubtype::HiLo => {
                let data = unwrap_node_data!(node, NodeData::SliceHiLo);

                let high = self.translate_expression(&data.hi_expr, symbol_table, false)?;
                let low = self.translate_expression(&data.lo_expr, symbol_table, false)?;

                Ok(quote! { #expression.extract_slice(integer_to_usize!(#low), integer_to_usize!(#high) + 1 - integer_to_usize!(#low))? })
            }
            NodeSubtype::LoWd => {
                let data = unwrap_node_data!(node, NodeData::SliceLoWd);

                let low = self.translate_expression(&data.lo_expr, symbol_table, false)?;
                let width = self.translate_expression(&data.wd_expr, symbol_table, false)?;

                Ok(quote! { #expression.extract_slice(integer_to_usize!(#low), integer_to_usize!(#width))? })
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_t_callable(&mut self, call_data: &StmtTCallData) -> Result<(), CodegenError> {
        let normalized_ident = AstAnalyzer::normalize_ident(&call_data.ident);
        let procedure_name = Ident::new(&normalized_ident, Span::call_site());

        if self.helpers.contains_key(&normalized_ident) {
            return Ok(());
        }

        let analyzer = Rc::clone(&self.analyzer);
        let procedure_node = analyzer.get_procedure(&call_data.ident)?;

        let procedure = match self.translate_procedure_definition(procedure_node) {
            Ok(tokens) => tokens,
            Err(CodegenError::NotImplemented { .. }) => quote! {
                /// Types for this procedure signature aren't implemented
                fn #procedure_name() {}
            },
            Err(e) => return Err(e),
        };

        self.helpers.insert(normalized_ident, Some(procedure));

        Ok(())
    }

    fn translate_procedure_definition(&mut self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::ProcDefn => {
                let data = unwrap_node_data!(node, NodeData::DeclProcDefn);

                let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);
                let procedure_name = Ident::new(&normalized_ident, Span::call_site());
                let procedure_name_lit_str = LitStr::new(&normalized_ident, Span::call_site());

                let mut symbol_table = BTreeMap::new();

                let params = self.translate_helper_params(&data.params, None, &normalized_ident, &mut symbol_table)?;

                let body = self.translate_helper_body(&data.stmts, &mut symbol_table)?;

                Ok(quote! {
                    pub fn #procedure_name(#(#params),*) -> Result<(), AArch64LifterError> {
                        #body
                        return Err(AArch64LifterError::NothingToReturn(#procedure_name_lit_str.to_string()));
                    }
                })
            }
            _ => panic!("Unexpected node type: type={:?} subtype={:?}", node.node_type, node.node_subtype),
        }
    }

    fn translate_t_applicable(&mut self, apply_data: &ExprTApplyData) -> Result<(), CodegenError> {
        let normalized_ident = AstAnalyzer::normalize_ident(&apply_data.ident);
        let function_name = Ident::new(&normalized_ident, Span::call_site());

        if self.helpers.contains_key(&normalized_ident) {
            return Ok(());
        }

        let analyzer = Rc::clone(&self.analyzer);
        let function_node = analyzer.get_function(&apply_data.ident)?;

        let function = match self.translate_function_definition(function_node) {
            Ok(tokens) => tokens,
            Err(CodegenError::NotImplemented { .. }) => quote! {
                /// Types for this function signature aren't implemented
                fn #function_name() {}
            },
            Err(e) => return Err(e),
        };

        self.helpers.insert(normalized_ident, Some(function));

        Ok(())
    }

    fn translate_function_definition(&mut self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::BuiltinFunction => {
                let data = unwrap_node_data!(node, NodeData::DeclBuiltinFunction);

                Err(CodegenError::MissingBuiltinFunctionImplementation(data.ident.to_string()))
            }
            NodeSubtype::FunDefn => {
                let data = unwrap_node_data!(node, NodeData::DeclFunDefn);

                let return_type = self.translate_type(&data.ty)?;

                let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);
                let function_name = Ident::new(&normalized_ident, Span::call_site());
                let function_name_lit_str = LitStr::new(&normalized_ident, Span::call_site());

                let mut symbol_table = BTreeMap::new();

                let params = self.translate_helper_params(&data.params, Some(&data.ty), &normalized_ident, &mut symbol_table)?;

                let body = self.translate_helper_body(&data.stmts, &mut symbol_table)?;

                Ok(quote! {
                    pub fn #function_name(#(#params),*) -> Result<#return_type, AArch64LifterError> {
                        #body
                        return Err(AArch64LifterError::NothingToReturn(#function_name_lit_str.to_string()));
                    }
                })
            }
            _ => panic!("Unexpected node type: type={:?} subtype={:?}", node.node_type, node.node_subtype),
        }
    }

    fn translate_helper_params(
        &mut self,
        params_data: &Vec<TypedIdentifier>,
        return_type: Option<&AstNode>,
        helper_name: &str,
        symbol_table: &mut BTreeMap<String, VariableMetadata>,
    ) -> Result<Vec<TokenStream>, CodegenError> {
        let mut params = Vec::new();
        for param in params_data.iter() {
            let param_type = self.translate_type(&param.ty)?;
            let param_name = Ident::new(&param.ident, Span::call_site());
            params.push(quote! { #param_name: #param_type });
            symbol_table.insert(
                param.ident.clone(),
                VariableMetadata {
                    type_name: self.get_type_name(&param.ty)?,
                },
            );
        }

        let implicit_params = self.get_implicit_params(params_data, return_type, symbol_table)?;
        let implicit_param_count = implicit_params.len();

        self.helper_table.insert(
            helper_name.to_string(),
            HelperMetadata {
                is_builtin: false,
                implicit_param_count,
            },
        );

        params.extend(implicit_params);

        Ok(params)
    }

    fn translate_helper_body(
        &mut self,
        statement_nodes: &Vec<AstNode>,
        symbol_table: &mut BTreeMap<String, VariableMetadata>,
    ) -> Result<TokenStream, CodegenError> {
        let mut statements = Vec::new();
        for stmt in statement_nodes.iter() {
            match self.translate_statement(stmt, symbol_table) {
                Ok(tokens) => statements.push(tokens),
                Err(CodegenError::NotImplemented(file, line)) => {
                    let line = line as usize;
                    statements.push(quote! {
                        return Err(AArch64LifterError::UnimplementedInstruction(#file.to_string(), #line));
                    });
                }
                Err(CodegenError::MissingBuiltinFunctionImplementation(function_name)) => {
                    statements.push(quote! {
                        return Err(AArch64LifterError::UnimplementedBuiltinFunction(#function_name.to_string()));
                    });
                }
                Err(e) => return Err(e),
            }
        }

        Ok(quote! { #(#statements)* })
    }

    fn get_implicit_params(
        &mut self,
        params: &Vec<TypedIdentifier>,
        opt_return_type: Option<&AstNode>,
        symbol_table: &mut BTreeMap<String, VariableMetadata>,
    ) -> Result<Vec<TokenStream>, CodegenError> {
        let mut param_names = params.iter().map(|param| param.ident.clone()).collect::<Vec<String>>();
        let explicit_param_count = param_names.len();

        let mut implicit_param_names = Vec::new();
        for param in params.iter() {
            implicit_param_names.extend(self.get_implicit_params_inner(&param.ty)?);
        }
        if let Some(return_type) = opt_return_type {
            implicit_param_names.extend(self.get_implicit_params_inner(return_type)?);
        }
        self.remove_duplicate_param_names(&mut implicit_param_names)?;
        self.rename_duplicate_param_names(&param_names, &mut implicit_param_names)?;
        param_names.extend(implicit_param_names);

        let renamed_implicit_param_names = param_names[explicit_param_count..].to_vec();

        let implicit_params = renamed_implicit_param_names
            .iter()
            .map(|param_name| {
                let param_ident = Ident::new(param_name, Span::call_site());
                symbol_table.insert(
                    param_name.to_string(),
                    VariableMetadata {
                        type_name: "integer".to_string(),
                    },
                );
                quote! { #param_ident: common::types::integer }
            })
            .collect::<Vec<TokenStream>>();

        Ok(implicit_params)
    }

    fn get_implicit_params_inner(&mut self, return_type: &AstNode) -> Result<Vec<String>, CodegenError> {
        match return_type.node_subtype {
            NodeSubtype::Constructor => Ok(vec![]),
            NodeSubtype::Bits => {
                let data = unwrap_node_data!(return_type, NodeData::TypeBits);

                let implicit_params = self.get_implicit_params_from_expression(&data.expr)?;

                Ok(implicit_params)
            }
            NodeSubtype::App => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::OfExpr => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Register => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(return_type, NodeData::TypeTuple);

                let implicit_params = data
                    .tys
                    .iter()
                    .map(|ty| self.get_implicit_params_inner(ty))
                    .collect::<Result<Vec<Vec<String>>, CodegenError>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<String>>();

                Ok(implicit_params)
            }
            _ => Err(CodegenError::InvalidNodeType(return_type.node_type, return_type.node_subtype)),
        }
    }

    fn get_implicit_params_from_expression(&self, node: &AstNode) -> Result<Vec<String>, CodegenError> {
        match node.node_subtype {
            NodeSubtype::If => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Binop => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Unop => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Field => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Fields => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Slices => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::In => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Var => {
                let data = unwrap_node_data!(node, NodeData::ExprVar);

                let implicit_param_name = &data.ident;

                Ok(vec![implicit_param_name.to_string()])
            }
            NodeSubtype::Parens => {
                let data = unwrap_node_data!(node, NodeData::ExprParens);

                let implicit_params = self.get_implicit_params_from_expression(&data.expr)?;

                Ok(implicit_params)
            }
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(node, NodeData::ExprTuple);

                let implicit_params = data
                    .exprs
                    .iter()
                    .map(|expression| self.get_implicit_params_from_expression(expression))
                    .collect::<Result<Vec<Vec<String>>, CodegenError>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<String>>();

                Ok(implicit_params)
            }
            NodeSubtype::Unknown => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ImpDef => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::TApply => {
                let data = unwrap_node_data!(node, NodeData::ExprTApply);

                if data.exprs1.len() > 0 {
                    println!("cargo:warning=implicit arguments present in TApply within function signature")
                }

                let implicit_params = data
                    .exprs2
                    .iter()
                    .map(|expression| self.get_implicit_params_from_expression(expression))
                    .collect::<Result<Vec<Vec<String>>, CodegenError>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<String>>();

                Ok(implicit_params)
            }
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitInt => Ok(vec![]),
            NodeSubtype::LitHex => Ok(vec![]),
            NodeSubtype::LitReal => Ok(vec![]),
            NodeSubtype::LitBits => Ok(vec![]),
            NodeSubtype::LitMask => Ok(vec![]),
            NodeSubtype::LitString => Ok(vec![]),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn remove_duplicate_param_names(&self, param_names: &mut Vec<String>) -> Result<(), CodegenError> {
        let mut seen = BTreeSet::new();
        param_names.retain(|name| seen.insert(name.clone()));
        Ok(())
    }

    fn rename_duplicate_param_names(&self, explicit_param_names: &Vec<String>, implicit_param_names: &mut Vec<String>) -> Result<(), CodegenError> {
        for name in implicit_param_names.iter_mut() {
            if explicit_param_names.contains(name) {
                *name = "_".to_string();
            }
        }

        Ok(())
    }
}
