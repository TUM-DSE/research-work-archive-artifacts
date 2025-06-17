use crate::asl::ast::{AstAnalyzer, AstNode, InstructionSet, NodeData, NodeSubtype, OperandMetadata, TypedIdentifier};
use crate::asl::CodegenError;
use crate::unwrap_node_data;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;
use std::{fs, iter};
use syn::{parse_file, Item, LitInt, LitStr};

#[derive(Debug, Clone)]
struct VariableMetadata {
    air_assigned: bool,
}

#[derive(Debug, Clone)]
struct HelperMetadata {
    is_builtin: bool,
    implicit_param_count: usize,
}

#[derive(Debug, Clone)]
struct ScopeContext {
    symbol_table: BTreeMap<String, VariableMetadata>,
    promoted_flow: bool,
    branch_returned: bool,
    level: usize,
}

#[derive(Debug)]
struct FunctionContext {
    return_type_group: FunctionReturnTypeGroup,
    return_type: Option<AstNode>,
}

#[derive(Debug, Clone, Copy)]
enum FunctionReturnTypeGroup {
    None,
    Single,
    Tuple(usize),
}

pub struct LiftTranspiler {
    analyzer: Rc<AstAnalyzer>,

    supported_opcodes: BTreeSet<String>,
    opcodes_and_operands: BTreeMap<String, Vec<OperandMetadata>>,
    enum_types: BTreeSet<String>,
    enum_variants: BTreeMap<String, String>,
    constant_names: BTreeSet<String>,
    records: BTreeMap<String, Vec<TypedIdentifier>>,

    lift_blocks: BTreeMap<String, TokenStream>,
    helpers: BTreeMap<String, Option<TokenStream>>,
    helper_table: BTreeMap<String, HelperMetadata>,
    helpers_in_translation: BTreeSet<String>,
}

impl LiftTranspiler {
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
        let records = analyzer.get_records()?.clone();

        Ok(Self {
            analyzer,

            supported_opcodes,
            opcodes_and_operands,
            enum_types,
            enum_variants,
            constant_names,
            records,

            lift_blocks: BTreeMap::new(),
            helpers,
            helper_table,
            helpers_in_translation: BTreeSet::new(),
        })
    }

    pub fn translate_records(&mut self) -> Result<TokenStream, CodegenError> {
        let mut translated_records = TokenStream::new();

        let mut packable_variants = Vec::new();
        let mut unpack_branches = Vec::new();
        let mut pack_branches = Vec::new();
        let mut packable_traits = Vec::new();

        for (name, fields) in self.records.clone().iter() {
            let record_name = &AstAnalyzer::normalize_ident(name);
            let normalized_ident = Ident::new(&record_name, Span::call_site());
            let normalized_lit_str = LitStr::new(&record_name, Span::call_site());
            let mut fields_tokens = Vec::new();
            let mut fields_default_tokens = Vec::new();
            let mut fields_unpack = Vec::new();
            let mut fields_pack = Vec::new();
            let mut packed_fields = Vec::new();

            for field in fields {
                let field_ident = Ident::new(&field.ident, Span::call_site());
                let ty_tokens = self.translate_type(&field.ty)?;

                fields_tokens.push(quote! { pub #field_ident: Box<#ty_tokens> });
                let inner_ty = self.translate_inner_type(&field.ty)?;
                let default_init = self.translate_type_to_default_init(&field.ty, None, &inner_ty)?;
                fields_default_tokens.push(quote! { #field_ident: Box::new(#default_init) });

                let unpacked_values_field = Ident::new(&format!("unpacked_values_{}", &field.ident), Span::call_site());
                let unpacked_types_field = Ident::new(&format!("unpacked_types_{}", &field.ident), Span::call_site());
                fields_unpack.push(quote! {
                    let packable: AirPackable = (*#normalized_ident.#field_ident).clone().into();
                    let (#unpacked_values_field, #unpacked_types_field) = packable.unpack_to_air_values_and_types(builder)?;
                    unpacked_values.extend(#unpacked_values_field);
                    unpacked_types.extend(#unpacked_types_field);
                });
                let packed_field = Ident::new(&format!("packed_{}", &field.ident), Span::call_site());
                let consumed_field = Ident::new(&format!("consumed_{}", &field.ident), Span::call_site());
                fields_pack.push(quote! {
                    let packable: AirPackable = (*#normalized_ident.#field_ident).clone().into();
                    let (#packed_field, #consumed_field) = packable.pack_from_air_values_and_types(&values[consumed..], &types[consumed..])?;
                    consumed += #consumed_field;
                });
                packed_fields.push(quote! { #field_ident: Box::new(#packed_field.try_into()?) });
            }

            translated_records.extend(quote! {
                #[derive(Debug, Clone)]
                pub struct #normalized_ident {
                    #(#fields_tokens),*
                }
                impl Default for lift::types::#normalized_ident {
                    fn default() -> Self {
                        lift::types::#normalized_ident {
                            #(#fields_default_tokens),*
                        }
                    }
                }
            });

            packable_variants.push(quote! {
                #normalized_ident(Box<lift::types::#normalized_ident>)
            });

            unpack_branches.push(quote! {
                AirPackable::#normalized_ident(#normalized_ident) => {
                    let mut unpacked_values = Vec::new();
                    let mut unpacked_types = Vec::new();
                    #(#fields_unpack)*
                    Ok((unpacked_values, unpacked_types))
                }
            });

            pack_branches.push(quote! {
                AirPackable::#normalized_ident(#normalized_ident) => {
                    let #normalized_ident = &*#normalized_ident;
                    let mut consumed = 0;
                    #(#fields_pack)*
                    Ok((
                        #normalized_ident {
                            #(#packed_fields),*
                        }.into(),
                        consumed,
                    ))
                }
            });

            packable_traits.push(quote! {
                impl From<lift::types::#normalized_ident> for AirPackable {
                    fn from(inner: lift::types::#normalized_ident) -> Self {
                        AirPackable::#normalized_ident(Box::new(inner))
                    }
                }
                impl TryFrom<AirPackable> for lift::types::#normalized_ident {
                    type Error = AArch64LifterError;
                    fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
                        if let AirPackable::#normalized_ident(inner) = value {
                            Ok(*inner)
                        } else {
                            Err(AArch64LifterError::MismatchedAirPackableType(#normalized_lit_str.to_string(), format!("{:?}", value)))
                        }
                    }
                }
            });
        }

        let packable_declarations = quote! {
            #[derive(Debug, Clone)]
            pub enum AirPackable {
                Variable(Box<lift::types::Variable>),
                #(#packable_variants),*
            }
            impl AirPackable {
                pub fn unpack_to_air_values_and_types(&self, builder: &mut InstructionBuilder) -> Result<(Vec<Value>, Vec<Type>), AArch64LifterError> {
                    match self {
                        AirPackable::Variable(Variable) => {
                            let air_var = Variable.promote_to_air(builder)?.to_air()?;
                            Ok((vec![air_var.val], vec![air_var.ty]))
                        }
                        #(#unpack_branches)*
                    }
                }
                pub fn pack_from_air_values_and_types(&self, values: &[Value], types: &[Type]) -> Result<(Self, usize), AArch64LifterError> {
                    match self {
                        AirPackable::Variable(Variable) => Ok((lift::types::Variable::new_air(values[0], types[0]).into(), 1)),
                        #(#pack_branches)*
                    }
                }
            }
            impl From<lift::types::Variable> for AirPackable {
                fn from(inner: lift::types::Variable) -> Self {
                    AirPackable::Variable(Box::new(inner))
                }
            }
            impl TryFrom<AirPackable> for lift::types::Variable {
                type Error = AArch64LifterError;
                fn try_from(value: AirPackable) -> Result<Self, Self::Error> {
                    if let AirPackable::Variable(inner) = value {
                        Ok(*inner)
                    } else {
                        Err(AArch64LifterError::MismatchedAirPackableType("Variable".to_string(), format!("{:?}", value)))
                    }
                }
            }
            #(#packable_traits)*
        };

        Ok(quote! {
            #translated_records
            #packable_declarations
        })
    }

    pub fn translate_lift_logic(&mut self) -> Result<TokenStream, CodegenError> {
        let mut match_arms = Vec::new();

        for instruction_name in self.opcodes_and_operands.clone().keys() {
            let analyzer = Rc::clone(&self.analyzer);
            let instruction_node = analyzer.get_instruction_def_by_instruction_name(instruction_name)?;
            let instruction_data = unwrap_node_data!(instruction_node, NodeData::DeclInstructionDefn);

            let opcode_ident = Ident::new(instruction_name, Span::call_site());
            let supported = self.supported_opcodes.contains(instruction_name);
            let match_arm_body = self.translate_execute_block(instruction_name, &instruction_data.stmts, supported)?;
            match_arms.push(quote! {
                common::types::Instruction::#opcode_ident(operands) => {
                    // println!("{:?}", operands);
                    #match_arm_body
                }
            });
        }

        Ok(quote! {
            common::types::Instruction::NOP => {}
            common::types::Instruction::UNPRED | common::types::Instruction::UNALLOC | common::types::Instruction::UNDEF => {
                builder.trap();
            }
            #(#match_arms)*
            _ => return Err(AArch64LifterError::UnspecifiedInstruction),
        })
    }

    pub fn translate_sequencer_logic(&mut self) -> Result<TokenStream, CodegenError> {
        let mut match_arms = Vec::new();

        for instruction_name in self.opcodes_and_operands.clone().keys() {
            let opcode_ident = Ident::new(instruction_name, Span::call_site());
            let lift_function_name = Ident::new(&format!("lift_{}", instruction_name), Span::call_site());
            let operands_struct_ident = Ident::new(&format!("{}_operands", instruction_name), Span::call_site());
            let operands = self
                .opcodes_and_operands
                .get(instruction_name)
                .ok_or(CodegenError::MissingOperandAnalysisForInstruction(instruction_name.to_string()))?;
            let operand_field_idents = operands
                .iter()
                .map(|operand| Ident::new(&operand.name, Span::call_site()))
                .collect::<Vec<Ident>>();
            let match_arm_body = quote! {
                let common::types::#operands_struct_ident { #(#operand_field_idents),* } = *operands;
                lift::generated::lift_blocks::#lift_function_name(builder, sequencer, lift::types::Variable::from(common::types::bits::new(address as u128, 64)), #(#operand_field_idents),*)?;
            };
            match_arms.push(quote! {
                common::types::Instruction::NOP => {}
                common::types::Instruction::UNPRED | common::types::Instruction::UNALLOC | common::types::Instruction::UNDEF => {
                    builder.trap();
                }
                common::types::Instruction::#opcode_ident(operands) => {
                    // println!("{:?}", operands);
                    #match_arm_body
                }
            });
        }

        Ok(quote! {
            pub fn generated_sequencer_logic(instruction: common::types::Instruction, builder: &mut InstructionBuilder, sequencer: &mut BlockSequencer, address: u64) -> Result<(), AArch64LifterError> {
                match instruction {
                    #(#match_arms)*
                    _ => return Err(AArch64LifterError::UnspecifiedInstruction),
                };

                Ok(())
            }
        })
    }

    pub fn get_lift_blocks(&self) -> &BTreeMap<String, TokenStream> {
        &self.lift_blocks
    }

    pub fn get_helpers(&self) -> Vec<TokenStream> {
        self.helpers
            .values()
            .filter_map(|helper_opt| helper_opt.as_ref())
            .cloned()
            .collect::<Vec<TokenStream>>()
    }

    fn translate_execute_block(&mut self, instruction_name: &str, stmts: &Vec<AstNode>, supported: bool) -> Result<TokenStream, CodegenError> {
        let lift_function_name = Ident::new(&format!("lift_{}", instruction_name), Span::call_site());
        let operands_struct_ident = Ident::new(&format!("{}_operands", instruction_name), Span::call_site());

        let operands = self
            .opcodes_and_operands
            .get(instruction_name)
            .ok_or(CodegenError::MissingOperandAnalysisForInstruction(instruction_name.to_string()))?;
        let operand_field_idents = operands
            .iter()
            .map(|operand| Ident::new(&operand.name, Span::call_site()))
            .collect::<Vec<Ident>>();
        let operand_param_idents = operands
            .iter()
            .map(|operand| {
                let param_name = Ident::new(&operand.name, Span::call_site());
                let param_type = Ident::new(&operand.type_name, Span::call_site());
                quote! { #param_name: common::types::#param_type }
            })
            .collect::<Vec<TokenStream>>();

        let mut symbol_table = BTreeMap::new();
        for operand in operands.iter() {
            symbol_table.insert(
                operand.name.to_string(),
                VariableMetadata {
                    // type_name: "Variable".to_string(),
                    air_assigned: false,
                },
            );
        }
        let mut scope_context = ScopeContext {
            symbol_table,
            promoted_flow: false,
            branch_returned: false,
            level: 0,
        };
        let mut function_context = FunctionContext {
            return_type_group: FunctionReturnTypeGroup::None,
            return_type: None,
        };
        let lift_statements = if supported {
            self.translate_statements_safe(stmts, &mut scope_context, &mut function_context)?
        } else {
            self.translate_statements_unsupported(stmts)?
        };

        self.lift_blocks.insert(
            instruction_name.to_string(),
            quote! {
                pub fn #lift_function_name(builder: &mut InstructionBuilder, sequencer: &mut BlockSequencer, pc: lift::types::Variable, #(#operand_param_idents),*) -> Result<(), AArch64LifterError> {
                    #(let mut #operand_field_idents: lift::types::Variable = #operand_field_idents.into();)*
                    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
                    #lift_statements
                    Ok(())
                }
            },
        );

        Ok(quote! {
            let common::types::#operands_struct_ident { #(#operand_field_idents),* } = *operands;
            lift::generated::lift_blocks::#lift_function_name(builder, sequencer, lift::types::Variable::from(common::types::bits::new(pc as u128, 64)), #(#operand_field_idents),*)?;
        })
    }

    fn translate_statements_safe(
        &mut self,
        stmts: &[AstNode],
        scope_context: &mut ScopeContext,
        function_context: &mut FunctionContext,
    ) -> Result<TokenStream, CodegenError> {
        match self.translate_statements(stmts, scope_context, function_context) {
            Ok(tokens) => Ok(tokens),
            Err(CodegenError::NotImplemented(file, line)) => Ok(quote! { panic!("Unimplemented instruction in {} at line {}", #file, #line); }),
            Err(e) => Err(e),
        }
    }

    fn translate_statements(
        &mut self,
        stmts: &[AstNode],
        scope_context: &mut ScopeContext,
        function_context: &mut FunctionContext,
    ) -> Result<TokenStream, CodegenError> {
        if let Some((node, rest)) = stmts.split_first() {
            let translated_node = match node.node_subtype {
                NodeSubtype::VarDeclsNoInit => {
                    let data = unwrap_node_data!(node, NodeData::StmtVarDeclsNoInit);

                    let normalized_idents = data
                        .idents
                        .iter()
                        .map(|ident| AstAnalyzer::normalize_ident(ident))
                        .collect::<Vec<String>>();

                    let ty = self.translate_type(&data.ty)?;
                    // let ty_name = self.get_type_name(&data.ty)?;
                    let name_idents = normalized_idents
                        .iter()
                        .map(|ident| Ident::new(&ident, Span::call_site()))
                        .collect::<Vec<Ident>>();

                    for ident in normalized_idents.iter() {
                        if self.records.contains_key(ident) {
                            return Err(CodegenError::NotImplemented(file!(), line!()));
                        }

                        scope_context.symbol_table.insert(
                            ident.to_string(),
                            VariableMetadata {
                                // type_name: ty_name.clone(),
                                air_assigned: false,
                            },
                        );
                    }

                    let inner_ty = self.translate_inner_type(&data.ty)?;
                    let default_init = self.translate_type_to_default_init(&data.ty, Some(scope_context), &inner_ty)?;

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

                    let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);

                    let ty = self.translate_type(&data.ty)?;
                    // let ty_name = self.get_type_name(&data.ty)?;
                    let name_ident = Ident::new(&normalized_ident, Span::call_site());

                    scope_context.symbol_table.insert(
                        normalized_ident,
                        VariableMetadata {
                            // type_name: ty_name,
                            air_assigned: false,
                        },
                    );

                    let value = self.translate_expression(&data.expr, scope_context, true)?;

                    Ok(quote! {
                        let mut #name_ident: #ty = #value;
                    })
                }
                NodeSubtype::ConstDecl => {
                    let data = unwrap_node_data!(node, NodeData::StmtConstDecl);

                    let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);

                    let ty = self.translate_type(&data.ty)?;
                    // let ty_name = self.get_type_name(&data.ty)?;
                    let name_ident = Ident::new(&normalized_ident, Span::call_site());

                    scope_context.symbol_table.insert(
                        normalized_ident,
                        VariableMetadata {
                            // type_name: ty_name,
                            air_assigned: false,
                        },
                    );

                    let value = self.translate_expression(&data.expr, scope_context, true)?;

                    Ok(quote! {
                        let #name_ident: #ty = #value;
                    })
                }
                NodeSubtype::Assign => {
                    let data = unwrap_node_data!(node, NodeData::StmtAssign);

                    match self.translate_register_write(node, scope_context)? {
                        Some(write) => Ok(write),
                        None => match self.translate_flag_write(node, scope_context)? {
                            Some(write) => Ok(write),
                            None => {
                                let write_node = &data.l_expr;

                                match write_node.node_subtype {
                                    NodeSubtype::Write => {
                                        let write_data = unwrap_node_data!(write_node, NodeData::LExprWrite);

                                        self.translate_t_callable(&write_data.ident)?;

                                        let normalized_ident = AstAnalyzer::normalize_ident(&write_data.ident);
                                        let procedure_name = Ident::new(&normalized_ident, Span::call_site());

                                        match self.helper_table.get(&normalized_ident) {
                                            Some(metadata) => {
                                                if !metadata.is_builtin && metadata.implicit_param_count != write_data.exprs1.len() {
                                                    return Err(CodegenError::WrongImplicitParamCount(
                                                        normalized_ident,
                                                        write_data.exprs1.len(),
                                                        metadata.implicit_param_count,
                                                    ));
                                                }
                                            }
                                            None => {}
                                        }

                                        let mut argument_names = vec![quote! { builder }, quote! { sequencer }, quote! { pc.clone() }];
                                        let mut argument_declarations = Vec::new();
                                        for (i, argument_node) in iter::once(&data.expr)
                                            .chain(write_data.exprs2.iter())
                                            .chain(write_data.exprs1.iter())
                                            .enumerate()
                                        {
                                            let argument_value = self.translate_expression(argument_node, scope_context, true)?;
                                            let argument_name = Ident::new(format!("arg_{}", i).as_str(), Span::call_site());
                                            argument_names.push(quote! { #argument_name });
                                            argument_declarations.push(quote! { let #argument_name = #argument_value; });
                                        }

                                        Ok(quote! {
                                            {
                                                #(#argument_declarations)*
                                                lift::helpers::#procedure_name(#(#argument_names),*)?;
                                            }
                                        })
                                    }
                                    _ => {
                                        let rhs = self.translate_expression(&data.expr, scope_context, true)?;
                                        let assignment = if scope_context.promoted_flow {
                                            self.translate_assignment_promoted(&data.l_expr, &rhs, scope_context)?
                                        } else {
                                            self.translate_assignment(&data.l_expr, &rhs, scope_context)?
                                        };

                                        Ok(quote! {
                                            #assignment
                                        })
                                    }
                                }
                            }
                        },
                    }
                }
                NodeSubtype::FunReturn => {
                    let data = unwrap_node_data!(node, NodeData::StmtFunReturn);

                    let value = self.translate_expression(&data.expr, scope_context, true)?;

                    let promoted_return = match function_context.return_type_group {
                        FunctionReturnTypeGroup::None => panic!("Invalid return type"),
                        FunctionReturnTypeGroup::Single => {
                            let return_type_name =
                                self.get_type_name(&function_context.return_type.clone().ok_or(CodegenError::InvalidReturnType)?)?;
                            if return_type_name == "Variable" {
                                quote! {
                                    {
                                        let return_air = #value.promote_to_air(builder)?.to_air()?;
                                        return_block_param_types = vec![return_air.ty];
                                        return_block_preds.insert(builder.current_block(), vec![return_air.val]);
                                    }
                                }
                            } else if self.records.contains_key(&return_type_name) {
                                quote! {
                                    return_value = #value.into();
                                    let (arg_vals, arg_tys) = return_value.unpack_to_air_values_and_types(builder)?;
                                    return_block_param_types = arg_tys;
                                    return_block_preds.insert(builder.current_block(), arg_vals);
                                }
                            } else {
                                return Err(CodegenError::NotImplemented(file!(), line!()));
                            }
                        }
                        FunctionReturnTypeGroup::Tuple(n) => {
                            // TODO: handle records in tuples
                            let inner_type_names =
                                self.get_tuple_inner_type_names(&function_context.return_type.clone().ok_or(CodegenError::InvalidReturnType)?)?;
                            if inner_type_names.iter().all(|s| s == "Variable") {
                                let mut return_param_type_declarations = Vec::new();
                                let mut return_param_types = Vec::new();
                                let mut return_arg_declarations = Vec::new();
                                let mut return_args = Vec::new();
                                for i in 0..n {
                                    let index_lit_int = LitInt::new(&i.to_string(), Span::call_site());
                                    let return_param_type_ident = Ident::new(&format!("return_param_type_{}", i), Span::call_site());
                                    let return_arg_ident = Ident::new(&format!("return_arg_{}", i), Span::call_site());
                                    return_param_type_declarations.push(
                                        quote! { let #return_param_type_ident = return_val.#index_lit_int.promote_to_air(builder)?.to_air()?.ty; },
                                    );
                                    return_param_types.push(quote! { #return_param_type_ident });
                                    return_arg_declarations
                                        .push(quote! { let #return_arg_ident = return_val.#index_lit_int.promote_to_air(builder)?.to_air()?.val; });
                                    return_args.push(quote! { #return_arg_ident });
                                }

                                quote! {
                                    {
                                        let return_val = #value;
                                        #(#return_param_type_declarations)*
                                        #(#return_arg_declarations)*
                                        return_block_param_types = vec![#(#return_param_types),*];
                                        return_block_preds.insert(builder.current_block(), vec![#(#return_args),*]);
                                    }
                                }
                            } else {
                                quote! { return Err(AArch64LifterError::NotImplemented(file!(), line!())); }
                            }
                        }
                    };

                    if scope_context.promoted_flow {
                        Ok(quote! {
                            has_promoted_returns = true;
                            #promoted_return
                        })
                    } else {
                        Ok(quote! {
                            if has_promoted_returns {
                                #promoted_return
                            } else {
                                return Ok(#value);
                            }
                        })
                    }
                }
                NodeSubtype::ProcReturn => Ok(quote! { return Ok(()); }),
                NodeSubtype::Assert => {
                    if scope_context.promoted_flow {
                        Ok(quote! {})
                    } else {
                        let data = unwrap_node_data!(node, NodeData::StmtAssert);

                        let expression = self.translate_expression(&data.expr, scope_context, false)?;

                        Ok(quote! {
                            match #expression {
                                lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) =>
                                    assert_eq!(b_inner, common::types::boolean::TRUE),
                                lift::types::Variable::Air(_) => {}
                                _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                            }
                        })
                    }
                }
                NodeSubtype::Unpred => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::ConstrainedUnpred => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::ImpDef => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::ExceptionTaken => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::DepUnpred => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::DepImpDef => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::DepUndefined => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::See => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::Throw => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::DecodeExecute => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::TCall => {
                    let data = unwrap_node_data!(node, NodeData::StmtTCall);

                    self.translate_t_callable(&data.ident)?;

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

                    let mut argument_names = vec![quote! { builder }, quote! { sequencer }, quote! { pc.clone() }];
                    let mut argument_declarations = Vec::new();
                    for (i, argument_node) in data.exprs2.iter().chain(data.exprs1.iter()).enumerate() {
                        let argument_value = self.translate_expression(argument_node, scope_context, true)?;
                        let argument_name = Ident::new(format!("arg_{}", i).as_str(), Span::call_site());
                        argument_names.push(quote! { #argument_name });
                        argument_declarations.push(quote! { let #argument_name = #argument_value; });
                    }

                    Ok(quote! {
                        {
                            #(#argument_declarations)*
                            lift::helpers::#procedure_name(#(#argument_names),*)?;
                        }
                    })
                }
                NodeSubtype::If => {
                    let data = unwrap_node_data!(node, NodeData::StmtIf);

                    self.translate_if_else_branch(&data.expr, &data.stmts1, &data.s_elsifs, &data.stmts2, scope_context, function_context)
                }
                NodeSubtype::Case => {
                    let data = unwrap_node_data!(node, NodeData::StmtCase);

                    let expression = self.translate_expression(&data.expr, scope_context, true)?;
                    let alts = data
                        .alts
                        .iter()
                        .enumerate()
                        .map(|(i, alt)| self.translate_alt(alt, &expression, i == 0, scope_context, function_context))
                        .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                    let default_statements = if let Some(stmts) = &data.opt_stmts {
                        let mut default_block_scope_context = ScopeContext {
                            symbol_table: scope_context.symbol_table.clone(),
                            promoted_flow: false,
                            branch_returned: false,
                            level: scope_context.level + 1,
                        };
                        self.translate_statements_safe(stmts, &mut default_block_scope_context, function_context)?
                    } else {
                        quote! {}
                    };
                    let assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level + 1), Span::call_site());
                    let prev_assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level), Span::call_site());

                    Ok(quote! {
                        {
                            let expr = #expression;
                            match &expr {
                                lift::types::Variable::Rust(_) => {
                                    #(#alts)*
                                    else {
                                        let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                        #default_statements
                                    }
                                }
                                lift::types::Variable::Air(_) => return Err(AArch64LifterError::NotImplemented(file!(), line!())),
                            }
                        }
                    })
                }
                NodeSubtype::For => {
                    let data = unwrap_node_data!(node, NodeData::StmtFor);

                    let loop_counter = AstAnalyzer::normalize_ident(&data.ident);
                    let loop_counter_ident = Ident::new(&loop_counter, Span::call_site());
                    let initial_value = self.translate_expression(&data.expr1, scope_context, true)?;
                    let final_value = self.translate_expression(&data.expr2, scope_context, true)?;
                    let (loop_condition, loop_update) = match data.direction.node_subtype {
                        NodeSubtype::Up => (
                            quote! {
                                {
                                    let arg_1 = #final_value;
                                    lift::helpers::le_int_0(builder, sequencer, pc.clone(), #loop_counter_ident.clone(), arg_1)?
                                } == lift::types::Variable::from(common::types::boolean::TRUE)
                            },
                            quote! {
                                #loop_counter_ident = {
                                    let arg_1 = lift::types::Variable::from(common::types::integer::one());
                                    lift::helpers::add_int_0(builder, sequencer, pc.clone(), #loop_counter_ident.clone(), arg_1)?
                                }
                            },
                        ),
                        NodeSubtype::Down => (
                            quote! {
                                {
                                    let arg_1 = #final_value;
                                    lift::helpers::ge_int_0(builder, sequencer, pc.clone(), #loop_counter_ident.clone(), arg_1)?
                                } == lift::types::Variable::from(common::types::boolean::TRUE)
                            },
                            quote! {
                                #loop_counter_ident = {
                                    let arg_1 = lift::types::Variable::from(common::types::integer::one());
                                    lift::helpers::sub_int_0(builder, sequencer, pc.clone(), #loop_counter_ident.clone(), arg_1)?
                                }
                            },
                        ),
                        _ => return Err(CodegenError::InvalidNodeType(data.direction.node_type, data.direction.node_subtype)),
                    };

                    let mut body_block_scope_context = ScopeContext {
                        symbol_table: scope_context.symbol_table.clone(),
                        promoted_flow: false,
                        branch_returned: false,
                        level: scope_context.level + 1,
                    };
                    body_block_scope_context.symbol_table.insert(
                        loop_counter.clone(),
                        VariableMetadata {
                            // type_name: "Variable".to_string(),
                            air_assigned: false,
                        },
                    );
                    let body = self.translate_statements_safe(&data.stmts, &mut body_block_scope_context, function_context)?;
                    let assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level + 1), Span::call_site());
                    let prev_assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level), Span::call_site());

                    Ok(quote! {
                        match #initial_value {
                            lift::types::Variable::Rust(lift::types::RustVariable::integer(n_i)) => match #final_value {
                                lift::types::Variable::Rust(lift::types::RustVariable::integer(yi)) => {
                                    let mut #loop_counter_ident = #initial_value;
                                    while #loop_condition {
                                        let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                        #body
                                        #loop_update
                                    }
                                }
                                lift::types::Variable::Air(ya) => panic!("Unimplemented instruction in {} at line {}", file!(), line!()),
                                _ => panic!("Variable not integer"),
                            }
                            lift::types::Variable::Air(_a_i) => panic!("Unimplemented instruction in {} at line {}", file!(), line!()),
                            _ => panic!("Variable not integer"),
                        }
                    })
                }
                NodeSubtype::While => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::Repeat => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::Try => Err(CodegenError::NotImplemented(file!(), line!())),
                _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
            }?;

            let translated_rest = self.translate_statements(rest, scope_context, function_context)?;

            Ok(quote! {
                #translated_node
                #translated_rest
            })
        } else {
            Ok(quote! {})
        }
    }

    fn translate_statements_unsupported(
        &mut self,
        stmts: &[AstNode],
    ) -> Result<TokenStream, CodegenError> {
        if let Some((node, rest)) = stmts.split_first() {
            let translated_node = match node.node_subtype {
                NodeSubtype::VarDeclsNoInit => Ok(quote! {}),
                NodeSubtype::VarDecl => Ok(quote! {}),
                NodeSubtype::ConstDecl => Ok(quote! {}),
                NodeSubtype::Assign => {
                    let data = unwrap_node_data!(node, NodeData::StmtAssign);

                    let write_node = &data.l_expr;

                    match write_node.node_subtype {
                        NodeSubtype::Write => {
                            let write_data = unwrap_node_data!(write_node, NodeData::LExprWrite);

                            if AstAnalyzer::normalize_ident(&write_data.ident) == "X_set_0" {
                                let mut dummy_context = ScopeContext {
                                    symbol_table: BTreeMap::new(),
                                    promoted_flow: false,
                                    branch_returned: false,
                                    level: 0,
                                };
                                let index = self.translate_expression(&write_data.exprs2[0], &mut dummy_context, true)?;

                                Ok(quote! {
                                    {
                                        let ty = Type::new_fixed_int(64).ok_or(AArch64LifterError::InvalidBitsLength)?;
                                        let opaque = builder.opaque(ty);
                                        let index = #index;
                                        builder.write_reg(opaque, Reg::new(integer_to_u32!(index.to_integer()?)), ty);
                                    }
                                })
                            } else {
                                Ok(quote! {})
                            }
                        }
                        _ => Ok(quote! {})
                    }
                }
                NodeSubtype::FunReturn => Ok(quote! {}),
                NodeSubtype::ProcReturn => Ok(quote! {}),
                NodeSubtype::Assert => Ok(quote! {}),
                NodeSubtype::Unpred => Ok(quote! {}),
                NodeSubtype::ConstrainedUnpred => Ok(quote! {}),
                NodeSubtype::ImpDef => Ok(quote! {}),
                NodeSubtype::ExceptionTaken => Ok(quote! {}),
                NodeSubtype::DepUnpred => Ok(quote! {}),
                NodeSubtype::DepImpDef => Ok(quote! {}),
                NodeSubtype::DepUndefined => Ok(quote! {}),
                NodeSubtype::See => Ok(quote! {}),
                NodeSubtype::Throw => Ok(quote! {}),
                NodeSubtype::DecodeExecute => Ok(quote! {}),
                NodeSubtype::TCall => Ok(quote! {}),
                NodeSubtype::If => {
                    let data = unwrap_node_data!(node, NodeData::StmtIf);

                    let mut branches = Vec::new();
                    branches.push(self.translate_statements_unsupported(&data.stmts1)?);
                    for elsif in data.s_elsifs.iter() {
                        let elsif_data = unwrap_node_data!(elsif, NodeData::SElsifCond);
                        branches.push(self.translate_statements_unsupported(&elsif_data.stmts)?);
                    }
                    branches.push(self.translate_statements_unsupported(&data.stmts2)?);

                    Ok(quote! { #(#branches)* })
                }
                NodeSubtype::Case => {
                    let data = unwrap_node_data!(node, NodeData::StmtCase);

                    let mut branches = Vec::new();
                    for alt in data.alts.iter() {
                        let alt_data = unwrap_node_data!(alt, NodeData::Alt);
                        branches.push(self.translate_statements_unsupported(&alt_data.stmts)?);
                    }
                    if let Some(stmts) = &data.opt_stmts {
                        branches.push(self.translate_statements_unsupported(stmts)?);
                    }

                    Ok(quote! { #(#branches)* })
                }
                NodeSubtype::For => {
                    let data = unwrap_node_data!(node, NodeData::StmtFor);

                    let body = self.translate_statements_unsupported(&data.stmts)?;

                    Ok(quote! { #body })
                }
                NodeSubtype::While => {
                    let data = unwrap_node_data!(node, NodeData::StmtWhile);

                    let body = self.translate_statements_unsupported(&data.stmts)?;

                    Ok(quote! { #body })
                }
                NodeSubtype::Repeat => Err(CodegenError::NotImplemented(file!(), line!())),
                NodeSubtype::Try => Err(CodegenError::NotImplemented(file!(), line!())),
                _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
            }?;

            let translated_rest = self.translate_statements_unsupported(rest)?;

            Ok(quote! {
                #translated_node
                #translated_rest
            })
        } else {
            Ok(quote! {})
        }
    }

    fn translate_expression(&mut self, node: &AstNode, scope_context: &mut ScopeContext, needs_clone: bool) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::If => {
                let data = unwrap_node_data!(node, NodeData::ExprIf);

                let if_condition = self.translate_expression(&data.if_cond_expr, scope_context, true)?;

                let then_body = self.translate_expression(&data.if_body_expr, scope_context, needs_clone)?;
                let else_body = self.translate_e_elsifs(&data.e_elsifs, &data.else_body_expr, scope_context, needs_clone)?;
                let rust_branch = quote! {
                    if b_inner == common::types::boolean::TRUE {
                        #then_body
                    } else {
                        #else_body
                    }
                };

                let air_branch = match self.translate_expression(&data.if_body_expr, scope_context, true) {
                    Ok(then_body_promoted) => match self.translate_e_elsifs(&data.e_elsifs, &data.else_body_expr, scope_context, true) {
                        Ok(else_body_promoted) => {
                            quote! {
                                {
                                    let then_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    let else_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.jumpif(a_inner.val, then_block, [], else_block, []);
                                    builder.set_insert_block(then_block);
                                    let then_body_promoted = #then_body_promoted;
                                    let (arg_then_vals, arg_tys) = {
                                        let packable: lift::types::AirPackable = then_body_promoted.clone().into();
                                        packable.unpack_to_air_values_and_types(builder)?
                                    };
                                    let end_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &arg_tys)?;
                                    builder.jump(end_block, arg_then_vals);
                                    builder.set_insert_block(else_block);
                                    let (arg_else_vals, _) = {
                                        let packable: lift::types::AirPackable = #else_body_promoted.into();
                                        packable.unpack_to_air_values_and_types(builder)?
                                    };
                                    builder.jump(end_block, arg_else_vals);
                                    builder.set_insert_block(end_block);
                                    let mut end_args = Vec::new();
                                    for i in 0..arg_tys.len() {
                                        end_args.push(Value::from(builder.get_block_param(end_block, i as u32)));
                                    }
                                    let packable: lift::types::AirPackable = then_body_promoted.into();
                                    let (end_body_packable, _) = packable.pack_from_air_values_and_types(&end_args, &arg_tys)?;
                                    // let end_body_packable = *end_body_packable;
                                    end_body_packable.try_into()?
                                }
                            }
                        }
                        Err(CodegenError::NotImplemented(file, line)) => {
                            quote! { panic!("Unimplemented instruction in {} at line {}", #file, #line); }
                        }
                        Err(e) => return Err(e),
                    },
                    Err(CodegenError::NotImplemented(file, line)) => {
                        quote! { panic!("Unimplemented instruction in {} at line {}", #file, #line); }
                    }
                    Err(e) => return Err(e),
                };

                Ok(quote! {
                    {
                        let cond = #if_condition;
                        match cond {
                            lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                                #rust_branch
                            }
                            lift::types::Variable::Air(a_inner) => {
                                #air_branch
                            }
                            _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                        }
                    }
                })
            }
            NodeSubtype::Binop => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Unop => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Field => match self.translate_flag_read(node)? {
                Some(read) => Ok(read),
                None => {
                    let data = unwrap_node_data!(node, NodeData::ExprField);

                    let lhs = self.translate_expression(&data.expr, scope_context, false)?;
                    let field_name_ident = Ident::new(&data.ident, Span::call_site());

                    if needs_clone {
                        Ok(quote! { (*#lhs.#field_name_ident).clone() })
                    } else {
                        Ok(quote! { (*#lhs.#field_name_ident) })
                    }
                }
            },
            NodeSubtype::Fields => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Slices => {
                let data = unwrap_node_data!(node, NodeData::ExprSlices);

                let expression = self.translate_expression(&data.expr, scope_context, false)?;

                if data.slices.is_empty() {
                    Ok(quote! { #expression })
                } else {
                    let (offset, length) = self.translate_slice(&data.slices[0], scope_context)?;

                    let mut aggregate = quote! {
                        {
                            let arg_0 = #offset;
                            let arg_1 = #length;
                            #expression.extract_slice(builder, arg_0, arg_1)?
                        }
                    };

                    let mut aggregate_length = length;

                    for slice in &data.slices[1..] {
                        let (offset, length) = self.translate_slice(slice, scope_context)?;

                        aggregate = quote! {
                            {
                                let arg_0 = #aggregate;
                                let arg_1 = {
                                    let arg_0 = #offset;
                                    let arg_1 = #length;
                                    #expression.extract_slice(builder, arg_0, arg_1)?
                                };
                                let arg_2 = #aggregate_length;
                                let arg_3 = #length;
                                lift::helpers::append_bits_0(builder, sequencer, pc.clone(), arg_0, arg_1, arg_2, arg_3)?
                            }
                        };

                        aggregate_length = quote! { #aggregate_length + #length };
                    }

                    Ok(aggregate)
                }
            }
            NodeSubtype::In => {
                let data = unwrap_node_data!(node, NodeData::ExprIn);

                let expr = self.translate_expression(&data.expr, scope_context, true)?;

                let pattern = self.translate_pattern(&data.pattern, scope_context)?;

                Ok(quote! {
                    {
                        let expr = #expr;
                        match &expr {
                            lift::types::Variable::Rust(_) => {
                                if #pattern.contains(&expr) {
                                    lift::types::Variable::from(common::types::boolean::TRUE)
                                } else {
                                    lift::types::Variable::from(common::types::boolean::FALSE)
                                }
                            }
                            lift::types::Variable::Air(_) => return Err(AArch64LifterError::NotImplemented(file!(), line!())),
                        }
                    }
                })
            }
            NodeSubtype::Var => {
                let data = unwrap_node_data!(node, NodeData::ExprVar);

                match self.translate_register_read(node, scope_context)? {
                    Some(read) => Ok(read),
                    None => {
                        let name = AstAnalyzer::normalize_ident(&data.ident);
                        let name_ident = Ident::new(&name, Span::call_site());

                        if let Some(enum_type) = self.enum_variants.get(&name) {
                            let enum_ident = Ident::new(enum_type, Span::call_site());

                            Ok(quote! { lift::types::Variable::from(common::types::#enum_ident::#name_ident) })
                        } else if self.constant_names.contains(&data.ident) {
                            Ok(quote! { lift::types::Variable::from(common::types::#name_ident.clone()?) })
                        } else if name == "InGuardedPage" {
                            Ok(quote! { lift::types::Variable::from(common::types::boolean::FALSE) })
                        } else if name == "__Memory" {
                            Ok(quote! { common::types::Dummy })
                        } else if name == "_FFR" {
                            Ok(quote! { lift::types::Variable::from(common::types::bits::new(0, 256)) })
                        } else if scope_context.promoted_flow {
                            let name_lit_str = LitStr::new(&name, Span::call_site());
                            let assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level), Span::call_site());

                            Ok(quote! {
                                match #assigns_ident.get(#name_lit_str) {
                                    Some(packable) => (*packable).clone().try_into()?,
                                    None => #name_ident.clone(),
                                }
                            })
                        } else if needs_clone {
                            Ok(quote! { #name_ident.clone() })
                        } else {
                            Ok(quote! { #name_ident })
                        }
                    }
                }
            }
            NodeSubtype::Parens => {
                let data = unwrap_node_data!(node, NodeData::ExprParens);

                let expression = self.translate_expression(&data.expr, scope_context, needs_clone)?;

                Ok(quote! { (#expression) })
            }
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(node, NodeData::ExprTuple);

                let expressions = data
                    .exprs
                    .iter()
                    .map(|expression| self.translate_expression(expression, scope_context, needs_clone))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                Ok(quote! { (#(#expressions),*) })
            }
            NodeSubtype::Unknown => {
                let data = unwrap_node_data!(node, NodeData::ExprUnknown);

                let inner_ty = self.translate_inner_type(&data.ty)?;
                let dummy_init = self.translate_type_to_default_init(&data.ty, Some(scope_context), &inner_ty)?;

                Ok(quote! { #dummy_init })
            }
            NodeSubtype::ImpDef => {
                let data = unwrap_node_data!(node, NodeData::ExprImpDef);

                let global_variable = self.translate_type_to_implementation_defined(&data.ty)?;

                Ok(quote! { #global_variable })
            }
            NodeSubtype::TApply => {
                let data = unwrap_node_data!(node, NodeData::ExprTApply);

                self.translate_t_applicable(&data.ident)?;

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

                let mut argument_names = vec![quote! { builder }, quote! { sequencer }, quote! { pc.clone() }];
                let mut argument_declarations = Vec::new();
                for (i, argument_node) in data.exprs2.iter().chain(data.exprs1.iter()).enumerate() {
                    let argument_value = self.translate_expression(argument_node, scope_context, true)?;
                    let argument_name = Ident::new(format!("arg_{}", i).as_str(), Span::call_site());
                    argument_names.push(quote! { #argument_name });
                    argument_declarations.push(quote! { let #argument_name = #argument_value; });
                }

                Ok(quote! {
                    {
                        #(#argument_declarations)*
                        lift::helpers::#function_name(#(#argument_names),*)?
                    }
                })
            }
            NodeSubtype::Array => match self.translate_register_read(node, scope_context)? {
                Some(read) => Ok(read),
                None => Err(CodegenError::NotImplemented(file!(), line!())),
            },
            NodeSubtype::LitInt => {
                let data = unwrap_node_data!(node, NodeData::ExprLitInt);

                assert!(!data.int_lit.starts_with('-'));

                let lit_int = LitInt::new(&data.int_lit, Span::call_site());

                Ok(quote! { lift::types::Variable::from(common::types::integer::from(#lit_int)) })
            }
            NodeSubtype::LitHex => {
                let data = unwrap_node_data!(node, NodeData::ExprLitHex);

                let lit_str = LitStr::new(&data.hex_lit, Span::call_site());

                Ok(quote! { lift::types::Variable::from(common::types::integer::from(common::types::bits::from_hex_literal(#lit_str)?.value)) })
            }
            NodeSubtype::LitReal => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitBits => {
                let data = unwrap_node_data!(node, NodeData::ExprLitBits);

                let lit_str = LitStr::new(&data.bits_lit, Span::call_site());

                Ok(quote! { lift::types::Variable::from(common::types::bits::from_bits_literal(#lit_str)?) })
            }
            NodeSubtype::LitMask => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitString => {
                let data = unwrap_node_data!(node, NodeData::ExprLitString);

                let lit_str = LitStr::new(&data.string_lit, Span::call_site());

                Ok(quote! { #lit_str.to_string() })
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_left_expression(&mut self, node: &AstNode) -> Result<(TokenStream, String), CodegenError> {
        match node.node_subtype {
            NodeSubtype::Wildcard => Ok((quote! { _ }, "".to_string())),
            NodeSubtype::Var => {
                let data = unwrap_node_data!(node, NodeData::LExprVar);

                let name = AstAnalyzer::normalize_ident(&data.ident);
                let name_ident = Ident::new(&name, Span::call_site());

                Ok((quote! { #name_ident }, name.clone()))
            }
            NodeSubtype::Field => {
                let data = unwrap_node_data!(node, NodeData::LExprField);

                let field_name = AstAnalyzer::normalize_ident(&data.ident);
                let field_name_ident = Ident::new(&field_name, Span::call_site());

                let (lhs, var) = self.translate_left_expression(&data.l_expr)?;

                Ok((quote! { #lhs.#field_name_ident }, var))
            }
            NodeSubtype::Fields => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Slices => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::BitTuple => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Write => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ReadWrite => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_assignment(&mut self, node: &AstNode, rhs: &TokenStream, scope_context: &mut ScopeContext) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Wildcard => Ok(quote! { _ }),
            NodeSubtype::Var => {
                let data = unwrap_node_data!(node, NodeData::LExprVar);

                let name = AstAnalyzer::normalize_ident(&data.ident);

                if name == "InGuardedPage" {
                    return Ok(quote! {});
                }

                let name_ident = Ident::new(&name, Span::call_site());

                Ok(quote! { #name_ident = #rhs; })
            }
            NodeSubtype::Field => {
                let data = unwrap_node_data!(node, NodeData::LExprField);

                let (lhs, _) = self.translate_left_expression(&data.l_expr)?;
                let field_name = Ident::new(&data.ident, Span::call_site());

                Ok(quote! { #lhs.#field_name = Box::new(#rhs); })
            }
            NodeSubtype::Fields => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Slices => {
                let data = unwrap_node_data!(node, NodeData::LExprSlices);

                match data.slices.as_slice() {
                    [slice] => {
                        let (offset, length) = self.translate_slice(slice, scope_context)?;

                        let (lhs, _) = self.translate_left_expression(&data.l_expr)?;

                        Ok(quote! {
                            {
                                let arg_0 = #rhs;
                                let arg_1 = #offset;
                                let arg_2 = #length;
                                #lhs.assign_slice(builder, arg_0, arg_1, arg_2)?;
                            }
                        })
                    }
                    _ => Err(CodegenError::NotImplemented(file!(), line!())),
                }
            }
            NodeSubtype::BitTuple => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(node, NodeData::LExprTuple);

                let contents = data
                    .l_exprs
                    .iter()
                    .map(|expression| self.translate_left_expression(expression))
                    .collect::<Result<Vec<(TokenStream, String)>, CodegenError>>()?
                    .into_iter()
                    .map(|(tokens, _)| tokens)
                    .collect::<Vec<TokenStream>>();

                Ok(quote! { (#(#contents),*) = #rhs; })
            }
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Write => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ReadWrite => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_assignment_promoted(
        &mut self,
        node: &AstNode,
        rhs: &TokenStream,
        scope_context: &mut ScopeContext,
    ) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Wildcard => Ok(quote! {}),
            NodeSubtype::Var => {
                let data = unwrap_node_data!(node, NodeData::LExprVar);

                let name = AstAnalyzer::normalize_ident(&data.ident);
                let name_lit_str = LitStr::new(&name, Span::call_site());
                let assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level), Span::call_site());

                if name == "InGuardedPage" || name == "_FFR" {
                    return Ok(quote! {});
                }

                let metadata = scope_context
                    .symbol_table
                    .get_mut(&name)
                    .expect(&format!("Missing variable in symbol table {}", name));

                metadata.air_assigned = true;

                Ok(quote! { #assigns_ident.insert(#name_lit_str.to_string(), #rhs.into()); })
            }
            NodeSubtype::Field => {
                let data = unwrap_node_data!(node, NodeData::LExprField);

                let (lhs, var) = self.translate_left_expression(&data.l_expr)?;
                let var_ident = Ident::new(&var, Span::call_site());
                let var_lit_str = LitStr::new(&var, Span::call_site());
                let field_name = Ident::new(&data.ident, Span::call_site());
                let assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level), Span::call_site());

                Ok(quote! {
                    {
                        let mut #var_ident = #var_ident.clone();
                        #lhs.#field_name = Box::new(#rhs);
                        #assigns_ident.insert(#var_lit_str.to_string(), #var_ident.into());
                    }
                })
            }
            NodeSubtype::Fields => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Slices => {
                let data = unwrap_node_data!(node, NodeData::LExprSlices);

                match data.slices.as_slice() {
                    [slice] => {
                        let (offset, length) = self.translate_slice(slice, scope_context)?;

                        let lhs_node = &data.l_expr;
                        let name = match lhs_node.node_subtype {
                            NodeSubtype::Wildcard => return Err(CodegenError::NotImplemented(file!(), line!())),
                            NodeSubtype::Var => {
                                let data = unwrap_node_data!(lhs_node, NodeData::LExprVar);

                                AstAnalyzer::normalize_ident(&data.ident)
                            }
                            NodeSubtype::Field => return Err(CodegenError::NotImplemented(file!(), line!())),
                            NodeSubtype::Fields => return Err(CodegenError::NotImplemented(file!(), line!())),
                            NodeSubtype::Slices => return Err(CodegenError::NotImplemented(file!(), line!())),
                            NodeSubtype::BitTuple => return Err(CodegenError::NotImplemented(file!(), line!())),
                            NodeSubtype::Tuple => return Err(CodegenError::NotImplemented(file!(), line!())),
                            NodeSubtype::Array => return Err(CodegenError::NotImplemented(file!(), line!())),
                            NodeSubtype::Write => return Err(CodegenError::NotImplemented(file!(), line!())),
                            NodeSubtype::ReadWrite => return Err(CodegenError::NotImplemented(file!(), line!())),
                            _ => return Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
                        };
                        let name_ident = Ident::new(&name, Span::call_site());
                        let name_lit_str = LitStr::new(&name, Span::call_site());
                        let assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level), Span::call_site());

                        Ok(quote! {
                            {
                                let mut cloned = #name_ident.clone();
                                let arg_0 = #rhs;
                                let arg_1 = #offset;
                                let arg_2 = #length;
                                cloned.assign_slice(builder, arg_0, arg_1, arg_2)?;
                                #assigns_ident.insert(#name_lit_str.to_string(), cloned.into());
                            }
                        })
                    }
                    _ => Err(CodegenError::NotImplemented(file!(), line!())),
                }
            }
            NodeSubtype::BitTuple => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(node, NodeData::LExprTuple);

                let assigns = data
                    .l_exprs
                    .iter()
                    .enumerate()
                    .map(|(i, expression)| {
                        let index_lit_int = LitInt::new(&i.to_string(), Span::call_site());
                        self.translate_assignment_promoted(expression, &quote! { rhs.#index_lit_int }, scope_context)
                    })
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                Ok(quote! {
                    {
                        let rhs = #rhs;
                        #(#assigns)*
                    }
                })
            }
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Write => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::ReadWrite => Err(CodegenError::NotImplemented(file!(), line!())),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_type(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => {
                let data = unwrap_node_data!(node, NodeData::TypeConstructor);

                if data.ident == "integer" || self.enum_types.contains(&data.ident) {
                    Ok(quote! { lift::types::Variable })
                } else if data.ident == "real" {
                    Ok(quote! { lift::types::Variable })
                } else if data.ident == "string" {
                    Ok(quote! { String })
                } else if self.records.contains_key(&data.ident) {
                    let normalized_ident = Ident::new(&AstAnalyzer::normalize_ident(&data.ident), Span::call_site());

                    Ok(quote! { lift::types::#normalized_ident })
                } else {
                    panic!("Unhandled type {}", data.ident);
                }
            }
            NodeSubtype::Bits => Ok(quote! { lift::types::Variable }),
            NodeSubtype::App => {
                let data = unwrap_node_data!(node, NodeData::TypeApp);

                if data.ident == "__RAM" {
                    Ok(quote! { common::types::Dummy })
                } else {
                    Err(CodegenError::NotImplemented(file!(), line!()))
                }
            }
            NodeSubtype::OfExpr => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Register => {
                // All ASL registers are essentially 64-length bits type
                Ok(quote! { lift::types::Variable })
            }
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

    fn translate_inner_type(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => {
                let data = unwrap_node_data!(node, NodeData::TypeConstructor);

                if data.ident == "integer" || self.enum_types.contains(&data.ident) {
                    let name_ident = Ident::new(&data.ident, Span::call_site());

                    Ok(quote! { common::types::#name_ident })
                } else if data.ident == "real" {
                    Ok(quote! { common::types::integer })
                } else if self.records.contains_key(&data.ident) {
                    let normalized_ident = Ident::new(&AstAnalyzer::normalize_ident(&data.ident), Span::call_site());
                    Ok(quote! { lift::types::#normalized_ident })
                } else {
                    panic!("Unhandled type {}", data.ident);
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

    fn translate_type_to_implementation_defined(&self, node: &AstNode) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => {
                let data = unwrap_node_data!(node, NodeData::TypeConstructor);
                if &data.ident == "integer" {
                    Ok(quote! { lift::types::Variable::from(common::types::integer::from(48)) })
                } else if &data.ident == "boolean" {
                    Ok(quote! { lift::types::Variable::from(common::types::boolean::FALSE) })
                } else {
                    Err(CodegenError::NotImplemented(file!(), line!()))
                }
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

    fn translate_type_to_default_init(
        &mut self,
        node: &AstNode,
        opt_scope_context: Option<&mut ScopeContext>,
        ty_ident: &TokenStream,
    ) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => {
                let data = unwrap_node_data!(node, NodeData::TypeConstructor);

                if self.records.contains_key(&AstAnalyzer::normalize_ident(&data.ident)) {
                    Ok(quote! { #ty_ident::default() })
                } else {
                    Ok(quote! { lift::types::Variable::from(#ty_ident::default()) })
                }
            }
            NodeSubtype::Bits => {
                let data = unwrap_node_data!(node, NodeData::TypeBits);

                match opt_scope_context {
                    Some(scope_context) => {
                        let bit_length = self.translate_expression(&data.expr, scope_context, true)?;

                        Ok(quote! {
                            match #bit_length {
                                lift::types::Variable::Rust(lift::types::RustVariable::integer(i_inner)) => {
                                    common::types::bits::new(0, integer_to_usize!(i_inner)).into()
                                }
                                lift::types::Variable::Air(a_inner) => {
                                    lift::types::Variable::air_from_bits(
                                        builder,
                                        common::types::bits::from_bits_literal("0")?,
                                    )?
                                }
                                _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                            }
                        })
                    }
                    None => {
                        println!("{:?}", node);

                        let bit_length_data = unwrap_node_data!(&data.expr, NodeData::ExprLitInt);
                        let bit_length = LitInt::new(&bit_length_data.int_lit, Span::call_site());

                        Ok(quote! {
                            lift::types::Variable::from(common::types::bits::new(0, #bit_length))
                        })
                    }
                }
            }
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

                if data.ident == "integer" || self.enum_types.contains(&data.ident) {
                    Ok("Variable".to_string())
                } else if data.ident == "real" {
                    Ok("Variable".to_string())
                } else if self.records.contains_key(&data.ident) {
                    Ok(data.ident.to_string())
                } else {
                    panic!("Unhandled type {}", data.ident);
                }
            }
            NodeSubtype::Bits => Ok("Variable".to_string()),
            NodeSubtype::App => Ok("App".to_string()),
            NodeSubtype::OfExpr => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Register => Ok("Variable".to_string()),
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => Ok("Tuple".to_string()),
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn get_tuple_inner_type_names(&self, node: &AstNode) -> Result<Vec<String>, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::TypeTuple);

        data.tys
            .iter()
            .map(|ty| self.get_type_name(ty))
            .collect::<Result<Vec<String>, CodegenError>>()
    }

    fn translate_s_elsifs(
        &mut self,
        elsifs: &[AstNode],
        else_stmts: &[AstNode],
        scope_context: &mut ScopeContext,
        function_context: &mut FunctionContext,
    ) -> Result<TokenStream, CodegenError> {
        match elsifs {
            [node, rest_elsifs @ ..] => {
                let data = unwrap_node_data!(node, NodeData::SElsifCond);

                self.translate_if_else_branch(&data.expr, &data.stmts, rest_elsifs, else_stmts, scope_context, function_context)
            }
            [] => {
                let statements = self.translate_statements_safe(else_stmts, scope_context, function_context)?;

                Ok(quote! { #statements })
            }
        }
    }

    fn translate_e_elsifs(
        &mut self,
        elsifs: &[AstNode],
        else_body: &AstNode,
        scope_context: &mut ScopeContext,
        needs_clone: bool,
    ) -> Result<TokenStream, CodegenError> {
        match elsifs {
            [node, rest_elsifs @ ..] => {
                let data = unwrap_node_data!(node, NodeData::EElsifCond);

                let condition = self.translate_expression(&data.expr1, scope_context, true)?;
                let expression = self.translate_expression(&data.expr2, scope_context, needs_clone)?;

                let rest = self.translate_e_elsifs(rest_elsifs, else_body, scope_context, needs_clone)?;

                Ok(quote! {
                    {
                        let cond = #condition;
                        match &cond {
                            lift::types::Variable::Rust(_) => {
                                if cond == lift::types::Variable::from(common::types::boolean::TRUE) {
                                    #expression
                                } else {
                                    #rest
                                }
                            }
                            lift::types::Variable::Air(_) => return Err(AArch64LifterError::NotImplemented(file!(), line!())),
                        }
                    }
                })
            }
            [] => {
                let expression = self.translate_expression(else_body, scope_context, needs_clone)?;

                Ok(quote! { #expression })
            }
        }
    }

    fn translate_if_else_branch(
        &mut self,
        if_expr: &AstNode,
        then_stmts: &[AstNode],
        elsifs: &[AstNode],
        else_stmts: &[AstNode],
        scope_context: &mut ScopeContext,
        function_context: &mut FunctionContext,
    ) -> Result<TokenStream, CodegenError> {
        let assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level + 1), Span::call_site());
        let prev_assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level), Span::call_site());

        let if_condition = self.translate_expression(if_expr, scope_context, true)?;

        let mut then_block_scope_context = ScopeContext {
            symbol_table: scope_context.symbol_table.clone(),
            promoted_flow: scope_context.promoted_flow,
            branch_returned: false,
            level: scope_context.level + 1,
        };
        let mut else_block_scope_context = ScopeContext {
            symbol_table: scope_context.symbol_table.clone(),
            promoted_flow: scope_context.promoted_flow,
            branch_returned: false,
            level: scope_context.level + 1,
        };
        for (_, metadata) in then_block_scope_context
            .symbol_table
            .iter_mut()
            .chain(else_block_scope_context.symbol_table.iter_mut())
        {
            metadata.air_assigned = false;
        }
        let then_statements = self.translate_statements_safe(then_stmts, &mut then_block_scope_context, function_context)?;
        let else_statements = self.translate_s_elsifs(elsifs, else_stmts, &mut else_block_scope_context, function_context)?;
        let mut then_bestow_assigns = Vec::new();
        let mut else_bestow_assigns = Vec::new();
        if scope_context.promoted_flow {
            for (variable, _) in scope_context.symbol_table.iter() {
                let var_ident = Ident::new(variable, Span::call_site());
                let var_lit_str = LitStr::new(variable, Span::call_site());
                let then_assigned = then_block_scope_context.symbol_table.get(variable).unwrap().air_assigned;
                let else_assigned = else_block_scope_context.symbol_table.get(variable).unwrap().air_assigned;
                if then_assigned {
                    then_bestow_assigns.push(quote! {
                        #prev_assigns_ident.insert(#var_lit_str.to_string(), #assigns_ident.get(#var_lit_str).unwrap().clone());
                    });
                    if !else_assigned {
                        else_bestow_assigns.push(quote! {
                            #prev_assigns_ident.entry(#var_lit_str.to_string()).or_insert(#var_ident.clone().into());
                        });
                    }
                }
                if else_assigned {
                    else_bestow_assigns.push(quote! {
                        #prev_assigns_ident.insert(#var_lit_str.to_string(), #assigns_ident.get(#var_lit_str).unwrap().clone());
                    });
                    if !then_assigned {
                        then_bestow_assigns.push(quote! {
                            #prev_assigns_ident.entry(#var_lit_str.to_string()).or_insert(#var_ident.clone().into());
                        });
                    }
                }
            }
        }

        let rust_branch = quote! {
            if b_inner == common::types::boolean::TRUE {
                let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                #then_statements
                #(#then_bestow_assigns)*
            } else {
                let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                #else_statements
                #(#else_bestow_assigns)*
            }
        };

        let mut then_block_promoted_scope_context = ScopeContext {
            symbol_table: scope_context.symbol_table.clone(),
            promoted_flow: true,
            branch_returned: false,
            level: scope_context.level + 1,
        };
        let mut else_block_promoted_scope_context = ScopeContext {
            symbol_table: scope_context.symbol_table.clone(),
            promoted_flow: true,
            branch_returned: false,
            level: scope_context.level + 1,
        };
        for (_, metadata) in then_block_promoted_scope_context
            .symbol_table
            .iter_mut()
            .chain(else_block_promoted_scope_context.symbol_table.iter_mut())
        {
            metadata.air_assigned = false;
        }
        let air_branch = match self.translate_statements(then_stmts, &mut then_block_promoted_scope_context, function_context) {
            Ok(then_statements_promoted) => {
                match self.translate_s_elsifs(elsifs, else_stmts, &mut else_block_promoted_scope_context, function_context) {
                    Ok(else_statements_promoted) => {
                        if then_block_promoted_scope_context.branch_returned && else_block_promoted_scope_context.branch_returned {
                            quote! {
                                {
                                    let current_block = builder.current_block();
                                    let then_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.set_insert_block(then_block);
                                    let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                    #then_statements_promoted
                                    let else_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.set_insert_block(current_block);
                                    builder.jumpif(a_inner.val, then_block, [], else_block, []);
                                    builder.set_insert_block(else_block);
                                    let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                    #else_statements_promoted
                                }
                            }
                        } else if then_block_promoted_scope_context.branch_returned && !else_block_promoted_scope_context.branch_returned {
                            let mut restore_assigns = Vec::new();
                            let mut bestow_assigns = Vec::new();
                            for (variable, metadata) in then_block_promoted_scope_context.symbol_table.iter() {
                                if metadata.air_assigned {
                                    let var_ident = Ident::new(variable, Span::call_site());
                                    let var_lit_str = LitStr::new(variable, Span::call_site());
                                    if scope_context.promoted_flow {
                                        bestow_assigns.push(quote! {
                                            #prev_assigns_ident.insert(#var_lit_str.to_string(), #assigns_ident.get(#var_lit_str).unwrap().clone());
                                        });
                                    } else {
                                        restore_assigns.push(quote! {
                                            #var_ident = #assigns_ident.get(#var_lit_str).unwrap().try_into()?;
                                        });
                                    }
                                    scope_context.symbol_table.get_mut(variable).unwrap().air_assigned = true;
                                }
                            }

                            quote! {
                                {
                                    let current_block = builder.current_block();
                                    let then_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.set_insert_block(then_block);
                                    let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                    #then_statements_promoted
                                    let else_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.set_insert_block(current_block);
                                    builder.jumpif(a_inner.val, then_block, [], else_block, []);
                                    builder.set_insert_block(else_block);
                                    let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                    #else_statements_promoted
                                    #(#bestow_assigns)*
                                    #(#restore_assigns)*
                                }
                            }
                        } else if !then_block_promoted_scope_context.branch_returned && else_block_promoted_scope_context.branch_returned {
                            let mut restore_assigns = Vec::new();
                            let mut bestow_assigns = Vec::new();
                            for (variable, metadata) in else_block_promoted_scope_context.symbol_table.iter() {
                                if metadata.air_assigned {
                                    let var_ident = Ident::new(variable, Span::call_site());
                                    let var_lit_str = LitStr::new(variable, Span::call_site());
                                    if scope_context.promoted_flow {
                                        bestow_assigns.push(quote! {
                                            #prev_assigns_ident.insert(#var_lit_str.to_string(), #assigns_ident.get(#var_lit_str).unwrap().clone());
                                        });
                                    } else {
                                        restore_assigns.push(quote! {
                                            #var_ident = #assigns_ident.get(#var_lit_str).unwrap().try_into()?;
                                        });
                                    }
                                    scope_context.symbol_table.get_mut(variable).unwrap().air_assigned = true;
                                }
                            }

                            quote! {
                                {
                                    let current_block = builder.current_block();
                                    let else_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.set_insert_block(else_block);
                                    let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                    #else_statements_promoted
                                    let then_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.set_insert_block(current_block);
                                    builder.jumpif(a_inner.val, then_block, [], else_block, []);
                                    builder.set_insert_block(then_block);
                                    let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                    #then_statements_promoted
                                    #(#bestow_assigns)*
                                    #(#restore_assigns)*
                                }
                            }
                        } else {
                            let mut block_param_names = Vec::new();
                            for (variable, metadata) in then_block_promoted_scope_context
                                .symbol_table
                                .iter()
                                .chain(else_block_promoted_scope_context.symbol_table.iter())
                            {
                                if metadata.air_assigned && !block_param_names.contains(variable) && scope_context.symbol_table.contains_key(variable)
                                {
                                    block_param_names.push(variable.to_string());
                                    scope_context.symbol_table.get_mut(variable).unwrap().air_assigned = true;
                                }
                            }

                            let mut then_unpacked = Vec::new();
                            let mut else_unpacked = Vec::new();
                            let mut restore_assigns = Vec::new();
                            let mut bestow_assigns = Vec::new();
                            for block_param_name in block_param_names.iter() {
                                let then_metadata = then_block_promoted_scope_context
                                    .symbol_table
                                    .get(block_param_name)
                                    .expect(&format!("Missing variable in symbol table {}", block_param_name.clone()));

                                let else_metadata = else_block_promoted_scope_context
                                    .symbol_table
                                    .get(block_param_name)
                                    .expect(&format!("Missing variable in symbol table {}", block_param_name.clone()));

                                let param_ident = Ident::new(block_param_name, Span::call_site());
                                let param_lit_str = LitStr::new(block_param_name, Span::call_site());

                                if then_metadata.air_assigned {
                                    then_unpacked
                                        .push(quote! { #assigns_ident.get(#param_lit_str).unwrap().unpack_to_air_values_and_types(builder)? });
                                } else {
                                    then_unpacked.push(quote! {
                                        {
                                            let packable: lift::types::AirPackable = #param_ident.clone().into();
                                            packable.unpack_to_air_values_and_types(builder)?
                                        }
                                    });
                                }

                                if else_metadata.air_assigned {
                                    else_unpacked
                                        .push(quote! { #assigns_ident.get(#param_lit_str).unwrap().unpack_to_air_values_and_types(builder)? });
                                } else {
                                    else_unpacked.push(quote! {
                                        {
                                            let packable: lift::types::AirPackable = #param_ident.clone().into();
                                            packable.unpack_to_air_values_and_types(builder)?
                                        }
                                    });
                                }

                                if scope_context.promoted_flow {
                                    bestow_assigns.push(quote! {
                                        let packable: lift::types::AirPackable = #param_ident.clone().into();
                                        let (packed, consumed) = packable.pack_from_air_values_and_types(&end_args[consumed_total..], &block_param_types[consumed_total..])?;
                                        #prev_assigns_ident.insert(#param_lit_str.to_string(), packed);
                                        consumed_total += consumed;
                                    });
                                } else {
                                    restore_assigns.push(quote! {
                                        let packable: lift::types::AirPackable = #param_ident.clone().into();
                                        let (packed, consumed) = packable.pack_from_air_values_and_types(&end_args[consumed_total..], &block_param_types[consumed_total..])?;
                                        #param_ident = packed.try_into()?;
                                        consumed_total += consumed;
                                    });
                                }
                            }

                            quote! {
                                {
                                    let current_block = builder.current_block();
                                    let then_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.set_insert_block(then_block);
                                    let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                    #then_statements_promoted
                                    let (then_args, block_param_types): (Vec<Value>, Vec<Type>) = vec![#(#then_unpacked),*].into_iter().flat_map(|(args, tys): (Vec<Value>, Vec<Type>)| args.into_iter().zip(tys.into_iter())).unzip();
                                    let else_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &vec![])?;
                                    builder.set_insert_block(current_block);
                                    builder.jumpif(a_inner.val, then_block, [], else_block, []);
                                    builder.set_insert_block(else_block);
                                    let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                                    #else_statements_promoted
                                    let (else_args, _): (Vec<Value>, Vec<Type>) = vec![#(#else_unpacked),*].into_iter().flat_map(|(args, tys): (Vec<Value>, Vec<Type>)| args.into_iter().zip(tys.into_iter())).unzip();
                                    let end_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &block_param_types)?;
                                    builder.set_insert_block(then_block);
                                    builder.jump(end_block, then_args);
                                    builder.set_insert_block(else_block);
                                    builder.jump(end_block, else_args);
                                    builder.set_insert_block(end_block);
                                    let mut end_args = Vec::new();
                                    for i in 0..block_param_types.len() {
                                        end_args.push(Value::from(builder.get_block_param(end_block, i as u32)));
                                    }
                                    let mut consumed_total = 0;
                                    #(#bestow_assigns)*
                                    #(#restore_assigns)*
                                }
                            }
                        }
                    }
                    Err(CodegenError::NotImplemented(file, line)) => {
                        quote! { panic!("Unimplemented instruction in {} at line {}", #file, #line); }
                    }
                    Err(e) => return Err(e),
                }
            }
            Err(CodegenError::NotImplemented(file, line)) => {
                quote! { panic!("Unimplemented instruction in {} at line {}", #file, #line); }
            }
            Err(e) => return Err(e),
        };

        Ok(quote! {
            {
                let cond = #if_condition;
                match cond {
                    lift::types::Variable::Rust(lift::types::RustVariable::boolean(b_inner)) => {
                        #rust_branch
                    }
                    lift::types::Variable::Air(a_inner) => {
                        #air_branch
                    }
                    _ => return Err(AArch64LifterError::VariableNotExpectedEnum),
                }
            }
        })
    }

    fn translate_alt(
        &mut self,
        node: &AstNode,
        expression: &TokenStream,
        first: bool,
        scope_context: &mut ScopeContext,
        function_context: &mut FunctionContext,
    ) -> Result<TokenStream, CodegenError> {
        let data = unwrap_node_data!(node, NodeData::Alt);

        let mut patterns = Vec::new();
        for pattern_node in data.patterns.iter() {
            let pattern = self.translate_pattern(pattern_node, scope_context)?;
            patterns.push(quote! { #expression == #pattern });
        }
        let guard = match &data.opt_expr {
            Some(expr) => self.translate_expression(expr, scope_context, false)?,
            None => quote! { true },
        };

        let mut alt_block_scope_context = ScopeContext {
            symbol_table: scope_context.symbol_table.clone(),
            promoted_flow: false,
            branch_returned: false,
            level: scope_context.level + 1,
        };
        let statements = self.translate_statements_safe(&data.stmts, &mut alt_block_scope_context, function_context)?;
        let assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level + 1), Span::call_site());
        let prev_assigns_ident = Ident::new(&format!("assigns_{}", scope_context.level), Span::call_site());

        let cond_prefix = if first {
            quote! { if }
        } else {
            quote! { else if }
        };

        Ok(quote! {
            #cond_prefix (#(#patterns)||*) && #guard {
                let mut #assigns_ident: BTreeMap<String, lift::types::AirPackable> = #prev_assigns_ident.clone();
                #statements
            }
        })
    }

    fn translate_pattern(&mut self, node: &AstNode, scope_context: &mut ScopeContext) -> Result<TokenStream, CodegenError> {
        match node.node_subtype {
            NodeSubtype::LitInt => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitHex => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::LitBits => {
                let data = unwrap_node_data!(node, NodeData::PatLitBits);

                let lit_str = LitStr::new(&data.bits_lit, Span::call_site());

                Ok(quote! { lift::types::Variable::from(common::types::bits::from_bits_literal(#lit_str)?) })
            }
            NodeSubtype::LitMask => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Const => {
                let data = unwrap_node_data!(node, NodeData::PatConst);

                let name = AstAnalyzer::normalize_ident(&data.ident);
                let name_ident = Ident::new(&name, Span::call_site());

                if let Some(enum_type) = self.enum_variants.get(&name) {
                    let enum_ident = Ident::new(enum_type, Span::call_site());
                    Ok(quote! { lift::types::Variable::from(common::types::#enum_ident::#name_ident) })
                } else if self.constant_names.contains(&data.ident) {
                    Ok(quote! { lift::types::Variable::from(common::types::#name_ident.clone()?) })
                } else {
                    Err(CodegenError::NotImplemented(file!(), line!()))
                }
            }
            NodeSubtype::Wildcard => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Set => {
                let data = unwrap_node_data!(node, NodeData::PatSet);

                let patterns = data
                    .patterns
                    .iter()
                    .map(|pattern| self.translate_pattern(pattern, scope_context))
                    .collect::<Result<Vec<TokenStream>, CodegenError>>()?;

                Ok(quote! { [#(#patterns),*] })
            }
            NodeSubtype::Range => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Single => {
                let data = unwrap_node_data!(node, NodeData::PatSingle);

                let expr = self.translate_expression(&data.expr, scope_context, false)?;

                Ok(quote! { #expr })
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_slice(&mut self, node: &AstNode, scope_context: &mut ScopeContext) -> Result<(TokenStream, TokenStream), CodegenError> {
        match node.node_subtype {
            NodeSubtype::Single => {
                let data = unwrap_node_data!(node, NodeData::SliceSingle);

                let index = self.translate_expression(&data.expr, scope_context, true)?;

                Ok((index, quote! { lift::types::Variable::from(common::types::integer::one()) }))
            }
            NodeSubtype::HiLo => {
                let data = unwrap_node_data!(node, NodeData::SliceHiLo);

                let high = self.translate_expression(&data.hi_expr, scope_context, true)?;
                let low = self.translate_expression(&data.lo_expr, scope_context, true)?;

                Ok((
                    low.clone(),
                    quote! {
                        {
                            let arg_0 = {
                                let arg_0 = #high;
                                let arg_1 = lift::types::Variable::from(common::types::integer::one());
                                lift::helpers::add_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
                            };
                            let arg_1 = #low;
                            lift::helpers::sub_int_0(builder, sequencer, pc.clone(), arg_0, arg_1)?
                        }
                    },
                ))
            }
            NodeSubtype::LoWd => {
                let data = unwrap_node_data!(node, NodeData::SliceLoWd);

                let low = self.translate_expression(&data.lo_expr, scope_context, true)?;
                let width = self.translate_expression(&data.wd_expr, scope_context, true)?;

                Ok((low, width))
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }

    fn translate_register_read(&mut self, node: &AstNode, scope_context: &mut ScopeContext) -> Result<Option<TokenStream>, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Array => {
                let array_data = unwrap_node_data!(node, NodeData::ExprArray);

                let var_node = &array_data.expr1;

                match var_node.node_subtype {
                    NodeSubtype::Var => {
                        let var_data = unwrap_node_data!(var_node, NodeData::ExprVar);

                        let register_name = &var_data.ident;
                        if register_name == "_R" {
                            let register_name_ident = Ident::new(register_name, Span::call_site());
                            let index = self.translate_expression(&array_data.expr2, scope_context, true)?;

                            Ok(Some(quote! {
                                {
                                    let arg_0 = lift::types::Register::#register_name_ident(#index);
                                    lift::types::Variable::from_register(builder, arg_0)?
                                }
                            }))
                        } else {
                            Ok(None)
                        }
                    }
                    _ => Ok(None),
                }
            }
            NodeSubtype::Var => {
                let var_data = unwrap_node_data!(node, NodeData::ExprVar);

                let register_name = &var_data.ident;

                if [
                    "SP_EL0",
                    "SP_EL1",
                    "SP_EL2",
                    "SP_EL3",
                    "TCR_EL1",
                    "TCR_EL2",
                    "TCR_EL3",
                    "SCR",
                    "SCR_EL3",
                    "HCR_EL2",
                    "SCTLR_EL1",
                    "SCTLR_EL2",
                    "SCTLR_EL3",
                    "APIAKeyHi_EL1",
                    "APIAKeyLo_EL1",
                    "APIBKeyHi_EL1",
                    "APIBKeyLo_EL1",
                    "DBGOSDLR",
                    "OSDLR_EL1",
                    "DBGPRCR",
                    "DBGPRCR_EL1",
                    "MDSCR_EL1",
                    "EDSCR",
                    "OSLSR_EL1",
                    "ID_AA64DFR0_EL1",
                    "MDCR_EL2",
                    "MDCR_EL3",
                    "MPAMVPM0_EL2",
                    "MPAMVPMV_EL2",
                    "MPAM3_EL3",
                    "MPAMIDR_EL1",
                    "MPAMHCR_EL2",
                    "MPAM1_EL1",
                    "MPAM2_EL2",
                    "SCTLR",
                    "HSCTLR",
                    "HCR2",
                    "TTBR0_EL1",
                    "TTBR0_EL2",
                    "TTBR0_EL3",
                    "TTBR1_EL1",
                    "TTBR1_EL2",
                    "VTCR_EL2",
                    "VSTCR_EL2",
                    "VSTTBR_EL2",
                    "VTTBR_EL2",
                ]
                .contains(&register_name.as_str())
                {
                    let register_name_ident = Ident::new(register_name, Span::call_site());

                    Ok(Some(quote! {
                        {
                            let arg_0 = lift::types::Register::#register_name_ident;
                            lift::types::Variable::from_register(builder, arg_0)?
                        }
                    }))
                } else if register_name == "_PC" {
                    Ok(Some(quote! { pc.clone() }))
                } else if ["SPIDEN", "DBGEN"].contains(&register_name.as_str()) {
                    Ok(Some(quote! { lift::types::Variable::from(common::types::signal::LOW) }))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    fn translate_register_write(&mut self, node: &AstNode, scope_context: &mut ScopeContext) -> Result<Option<TokenStream>, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Assign => {
                let assign_data = unwrap_node_data!(node, NodeData::StmtAssign);

                let array_node = &assign_data.l_expr;

                match array_node.node_subtype {
                    NodeSubtype::Var => {
                        let var_data = unwrap_node_data!(array_node, NodeData::LExprVar);
                        let register_name = &var_data.ident;
                        if [
                            "SP_EL0",
                            "SP_EL1",
                            "SP_EL2",
                            "SP_EL3",
                            "BTypeNext",
                            "DLR",
                            "DLR_EL0",
                            "DSPSR",
                            "DSPSR_EL0",
                        ]
                        .contains(&register_name.as_str())
                        {
                            let register_name_ident = Ident::new(register_name, Span::call_site());
                            let value = self.translate_expression(&assign_data.expr, scope_context, true)?;

                            Ok(Some(quote! {
                                {
                                    let arg_0 = lift::types::Register::#register_name_ident;
                                    #value.to_register(builder, arg_0)?;
                                }
                            }))
                        } else if register_name == "_PC" {
                            let value = self.translate_expression(&assign_data.expr, scope_context, true)?;

                            Ok(Some(quote! {
                                match #value {
                                    lift::types::Variable::Rust(lift::types::RustVariable::bits(b_inner)) => {
                                        let block = sequencer.get_block(b_inner.value as u64, lift::types::BlockType::InterBlock, builder, &vec![])?;
                                        builder.jump(block, vec![]);
                                    }
                                    lift::types::Variable::Air(a_inner) => {
                                        builder.dynamic_jump(a_inner.val);
                                    }
                                    _ => panic!("Variable not bits"),
                                }
                            }))
                        } else {
                            Ok(None)
                        }
                    }
                    NodeSubtype::Slices => {
                        let slice_data = unwrap_node_data!(array_node, NodeData::LExprSlices);

                        if slice_data.l_expr.node_subtype == NodeSubtype::Var {
                            let var_data = unwrap_node_data!(slice_data.l_expr, NodeData::LExprVar);
                            let register_name = &var_data.ident;
                            if ["TFSR_EL1", "TFSR_EL2", "TFSR_EL3", "TFSRE0_EL1", "MPAMVPMV_EL2", "EDSCR", "DSPSR"].contains(&register_name.as_str())
                            {
                                let register_name_ident = Ident::new(register_name, Span::call_site());
                                let value = self.translate_expression(&assign_data.expr, scope_context, true)?;

                                Ok(Some(quote! {
                                    {
                                        let arg_0 = lift::types::Register::#register_name_ident;
                                        #value.to_register(builder, arg_0)?;
                                    }
                                }))
                            } else {
                                Ok(None)
                            }
                        } else {
                            Ok(None)
                        }
                    }
                    NodeSubtype::Array => {
                        let array_data = unwrap_node_data!(array_node, NodeData::LExprArray);

                        let var_node = &array_data.l_expr;

                        match var_node.node_subtype {
                            NodeSubtype::Var => {
                                let var_data = unwrap_node_data!(var_node, NodeData::LExprVar);
                                let register_name = &var_data.ident;
                                if register_name == "_R" {
                                    let register_name_ident = Ident::new(register_name, Span::call_site());
                                    let index = self.translate_expression(&array_data.expr, scope_context, true)?;
                                    let value = self.translate_expression(&assign_data.expr, scope_context, true)?;

                                    Ok(Some(quote! {
                                        {
                                            let arg_0 = lift::types::Register::#register_name_ident(#index);
                                            #value.to_register(builder, arg_0)?;
                                        }
                                    }))
                                } else {
                                    Ok(None)
                                }
                            }
                            _ => Ok(None),
                        }
                    }
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    // fn translate_register_write_unsupported(&mut self, node: &AstNode) -> Result<Option<TokenStream>, CodegenError> {
    //     match node.node_subtype {
    //         NodeSubtype::Assign => {
    //             let assign_data = unwrap_node_data!(node, NodeData::StmtAssign);
    //
    //             let array_node = &assign_data.l_expr;
    //
    //             match array_node.node_subtype {
    //                 NodeSubtype::Array => {
    //                     let array_data = unwrap_node_data!(array_node, NodeData::LExprArray);
    //
    //                     let var_node = &array_data.l_expr;
    //
    //                     match var_node.node_subtype {
    //                         NodeSubtype::Var => {
    //                             let var_data = unwrap_node_data!(var_node, NodeData::LExprVar);
    //                             let register_name = &var_data.ident;
    //                             if register_name == "_R" {
    //                                 let mut dummy_context = ScopeContext {
    //                                     symbol_table: BTreeMap::new(),
    //                                     promoted_flow: false,
    //                                     branch_returned: false,
    //                                     level: 0,
    //                                 };
    //                                 let index = self.translate_expression(&array_data.expr, &mut dummy_context, false)?;
    //
    //                                 Ok(Some(quote! {
    //                                     {
    //                                         let ty = Type::new_fixed_int(b.length as u8).ok_or(AArch64LifterError::InvalidBitsLength)?;
    //                                         let opaque = builder.opaque(ty);
    //                                         builder.write_reg(val, Reg::new(integer_to_u32!(#index.to_integer()?)), ty);
    //                                     }
    //                                 }))
    //                             } else {
    //                                 Ok(None)
    //                             }
    //                         }
    //                         _ => Ok(None),
    //                     }
    //                 }
    //                 _ => Ok(None),
    //             }
    //         }
    //         _ => Ok(None),
    //     }
    // }

    fn translate_flag_read(&self, node: &AstNode) -> Result<Option<TokenStream>, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Field => {
                let field_data = unwrap_node_data!(node, NodeData::ExprField);

                let var_node = &field_data.expr;

                match var_node.node_subtype {
                    NodeSubtype::Var => {
                        let var_data = unwrap_node_data!(var_node, NodeData::ExprVar);

                        if var_data.ident == "PSTATE" {
                            let flag_name = &field_data.ident;
                            let flag_name_ident = Ident::new(flag_name, Span::call_site());
                            if ["N", "Z", "C", "V"].contains(&flag_name.as_str()) {
                                Ok(Some(
                                    quote! { lift::types::Variable::from_flag(builder, lift::types::Flag::#flag_name_ident)? },
                                ))
                            } else if flag_name == "SP" {
                                Ok(Some(quote! { lift::types::Variable::from(common::types::bits::new(1, 1)) }))
                            } else if flag_name == "EL" {
                                Ok(Some(quote! { lift::types::Variable::from(common::types::bits::new(0, 2)) }))
                            } else if flag_name == "nRW" {
                                Ok(Some(quote! { lift::types::Variable::from(common::types::bits::new(0, 1)) }))
                            } else if flag_name == "M" {
                                Ok(Some(quote! { lift::types::Variable::from(common::types::bits::new(16, 5)) }))
                            } else if flag_name == "PAN" {
                                Ok(Some(quote! { lift::types::Variable::from(common::types::bits::new(0, 1)) }))
                            } else if flag_name == "UAO" {
                                Ok(Some(quote! { lift::types::Variable::from(common::types::bits::new(0, 1)) }))
                            } else if flag_name == "D" {
                                Ok(Some(quote! { lift::types::Variable::from(common::types::bits::new(0, 1)) }))
                            } else {
                                Err(CodegenError::NotImplemented(file!(), line!()))
                            }
                        } else {
                            Ok(None)
                        }
                    }
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    fn translate_flag_write(&mut self, node: &AstNode, scope_context: &mut ScopeContext) -> Result<Option<TokenStream>, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Assign => {
                let assign_data = unwrap_node_data!(node, NodeData::StmtAssign);

                let field_node = &assign_data.l_expr;

                match field_node.node_subtype {
                    NodeSubtype::Field => {
                        let field_data = unwrap_node_data!(field_node, NodeData::LExprField);

                        let var_node = &field_data.l_expr;

                        match var_node.node_subtype {
                            NodeSubtype::Var => {
                                let var_data = unwrap_node_data!(var_node, NodeData::LExprVar);

                                if var_data.ident == "PSTATE" {
                                    let expr = self.translate_expression(&assign_data.expr, scope_context, false)?;

                                    let flag = &field_data.ident;

                                    if [
                                        "N", "Z", "C", "V", "D", "I", "A", "F", "T", "IT", "IL", "SS", "EL", "nRW", "SP", "PAN", "UAO", "BTYPE",
                                        "SSBS", "TCO",
                                    ]
                                    .contains(&flag.as_str())
                                    {
                                        let flag_ident = Ident::new(flag, Span::call_site());
                                        Ok(Some(quote! { #expr.to_flag(builder, lift::types::Flag::#flag_ident); }))
                                    } else {
                                        panic!("{}", format!("Unrecognized flag {}", flag));
                                    }
                                } else {
                                    Ok(None)
                                }
                            }
                            _ => Ok(None),
                        }
                    }
                    NodeSubtype::Fields => {
                        let fields_data = unwrap_node_data!(field_node, NodeData::LExprFields);

                        let var_node = &fields_data.l_expr;

                        match var_node.node_subtype {
                            NodeSubtype::Var => {
                                let var_data = unwrap_node_data!(var_node, NodeData::LExprVar);

                                if var_data.ident == "PSTATE" {
                                    let expr = self.translate_expression(&assign_data.expr, scope_context, false)?;

                                    let mut flag_writes = quote! { let expr_ref = &(#expr); };
                                    let mut index = 0;
                                    for flag in fields_data.idents.iter().rev() {
                                        if ["N", "Z", "C", "V", "D", "I", "A", "F", "T", "IT", "IL", "SS"].contains(&flag.as_str()) {
                                            let index_lit_int = LitInt::new(&index.to_string(), Span::call_site());
                                            let flag_ident = Ident::new(flag, Span::call_site());
                                            flag_writes.extend(quote! {
                                                let flag_value = expr_ref.extract_slice(builder, common::types::integer::from(#index_lit_int).into(), common::types::integer::one().into())?;
                                                flag_value.to_flag(builder, lift::types::Flag::#flag_ident);
                                            });
                                            index += 1;
                                        } else {
                                            panic!("{}", format!("Unrecognized flag {}", flag));
                                        }
                                    }

                                    Ok(Some(quote! {
                                        {
                                            #flag_writes
                                        }
                                    }))
                                } else {
                                    Ok(None)
                                }
                            }
                            _ => Ok(None),
                        }
                    }
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    fn translate_t_callable(&mut self, name: &str) -> Result<(), CodegenError> {
        let normalized_ident = AstAnalyzer::normalize_ident(name);
        let procedure_name = Ident::new(&normalized_ident, Span::call_site());

        if self.helpers.contains_key(&normalized_ident) || self.helpers_in_translation.contains(&normalized_ident) {
            return Ok(());
        }

        self.helpers_in_translation.insert(normalized_ident.clone());

        let analyzer = Rc::clone(&self.analyzer);
        let procedure_node = analyzer.get_procedure(name)?;

        let procedure = match self.translate_procedure_definition(procedure_node) {
            Ok(tokens) => tokens,
            Err(CodegenError::NotImplemented { .. }) => quote! {
                /// Types for this procedure signature not implemented
                fn #procedure_name() {}
            },
            Err(e) => return Err(e),
        };

        self.helpers.insert(normalized_ident.clone(), Some(procedure));

        self.helpers_in_translation.remove(&normalized_ident);

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

                let body = self.translate_helper_body(&data.stmts, &mut symbol_table, FunctionReturnTypeGroup::None, None)?;

                Ok(quote! {
                    pub fn #procedure_name(#(#params),*) -> Result<(), AArch64LifterError> {
                        // println!(#procedure_name_lit_str);
                        #body
                        return Ok(());
                        return Err(AArch64LifterError::NothingToReturn(#procedure_name_lit_str.to_string()));
                    }
                })
            }
            NodeSubtype::ArraySetterDefn => {
                let data = unwrap_node_data!(node, NodeData::DeclArraySetterDefn);

                let normalized_ident = AstAnalyzer::normalize_ident(&data.ident1);
                let procedure_name = Ident::new(&normalized_ident, Span::call_site());
                let procedure_name_lit_str = LitStr::new(&normalized_ident, Span::call_site());

                let mut symbol_table = BTreeMap::new();

                let mut explicit_params = vec![TypedIdentifier {
                    ty: data.ty.clone(),
                    ident: AstAnalyzer::normalize_ident(&data.ident2),
                }];
                for additional_param in data.params.iter() {
                    let additional_param_data = unwrap_node_data!(additional_param, NodeData::FormalIn);
                    explicit_params.push(TypedIdentifier {
                        ty: additional_param_data.ty.clone(),
                        ident: AstAnalyzer::normalize_ident(&additional_param_data.ident),
                    });
                }
                let params = self.translate_helper_params(&explicit_params, None, &normalized_ident, &mut symbol_table)?;

                let body = self.translate_helper_body(&data.stmts, &mut symbol_table, FunctionReturnTypeGroup::None, None)?;

                Ok(quote! {
                    pub fn #procedure_name(#(#params),*) -> Result<(), AArch64LifterError> {
                        // println!(#procedure_name_lit_str);
                        #body
                        return Ok(());
                        return Err(AArch64LifterError::NothingToReturn(#procedure_name_lit_str.to_string()));
                    }
                })
            }
            _ => panic!("Unexpected node type: type={:?} subtype={:?}", node.node_type, node.node_subtype),
        }
    }

    fn translate_t_applicable(&mut self, name: &str) -> Result<(), CodegenError> {
        let normalized_ident = AstAnalyzer::normalize_ident(name);
        let function_name = Ident::new(&normalized_ident, Span::call_site());

        if self.helpers.contains_key(&normalized_ident) || self.helpers_in_translation.contains(&normalized_ident) {
            return Ok(());
        }

        self.helpers_in_translation.insert(normalized_ident.clone());

        let analyzer = Rc::clone(&self.analyzer);
        let function_node = analyzer.get_function(name)?;

        let function = match self.translate_function_definition(function_node) {
            Ok(tokens) => tokens,
            Err(CodegenError::NotImplemented { .. }) => quote! {
                /// Types for this function signature not implemented
                fn #function_name() {}
            },
            Err(e) => return Err(e),
        };

        self.helpers.insert(normalized_ident.clone(), Some(function));

        self.helpers_in_translation.remove(&normalized_ident);

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
                let return_type_enum = self.check_return_type_enum(&data.ty)?;

                let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);
                let function_name = Ident::new(&normalized_ident, Span::call_site());
                let function_name_lit_str = LitStr::new(&normalized_ident, Span::call_site());

                let mut symbol_table = BTreeMap::new();

                let params = self.translate_helper_params(&data.params, Some(&data.ty), &normalized_ident, &mut symbol_table)?;

                let body = self.translate_helper_body(&data.stmts, &mut symbol_table, return_type_enum, Some(&data.ty))?;

                Ok(quote! {
                    pub fn #function_name(#(#params),*) -> Result<#return_type, AArch64LifterError> {
                        // println!(#function_name_lit_str);
                        #body
                        return Err(AArch64LifterError::NothingToReturn(#function_name_lit_str.to_string()));
                    }
                })
            }
            NodeSubtype::ArrayGetterDefn => {
                let data = unwrap_node_data!(node, NodeData::DeclArrayGetterDefn);

                let return_type = self.translate_type(&data.ty)?;
                let return_type_enum = self.check_return_type_enum(&data.ty)?;

                let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);
                let function_name = Ident::new(&normalized_ident, Span::call_site());
                let function_name_lit_str = LitStr::new(&normalized_ident, Span::call_site());

                let mut symbol_table = BTreeMap::new();

                let params = self.translate_helper_params(&data.params, Some(&data.ty), &normalized_ident, &mut symbol_table)?;

                let body = self.translate_helper_body(&data.stmts, &mut symbol_table, return_type_enum, Some(&data.ty))?;

                Ok(quote! {
                    pub fn #function_name(#(#params),*) -> Result<#return_type, AArch64LifterError> {
                        // println!(#function_name_lit_str);
                        #body
                        return Err(AArch64LifterError::NothingToReturn(#function_name_lit_str.to_string()));
                    }
                })
            }
            NodeSubtype::VarGetterDefn => {
                let data = unwrap_node_data!(node, NodeData::DeclVarGetterDefn);

                let return_type = self.translate_type(&data.ty)?;
                let return_type_enum = self.check_return_type_enum(&data.ty)?;

                let normalized_ident = AstAnalyzer::normalize_ident(&data.ident);
                let function_name = Ident::new(&normalized_ident, Span::call_site());
                let function_name_lit_str = LitStr::new(&normalized_ident, Span::call_site());

                let mut symbol_table = BTreeMap::new();

                let body = self.translate_helper_body(&data.stmts, &mut symbol_table, return_type_enum, Some(&data.ty))?;

                Ok(quote! {
                    pub fn #function_name() -> Result<#return_type, AArch64LifterError> {
                        // println!(#function_name_lit_str);
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
        let mut params = vec![
            quote! { builder: &mut InstructionBuilder },
            quote! { sequencer: &mut BlockSequencer },
            quote! { pc: lift::types::Variable },
        ];
        for param in params_data.iter() {
            let param_type = self.translate_type(&param.ty)?;
            let param_name = AstAnalyzer::normalize_ident(&param.ident);
            let param_ident = Ident::new(&param_name, Span::call_site());
            params.push(quote! { mut #param_ident: #param_type });
            symbol_table.insert(param_name, VariableMetadata { air_assigned: false });
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
        return_type_group: FunctionReturnTypeGroup,
        return_type: Option<&AstNode>,
    ) -> Result<TokenStream, CodegenError> {
        let mut function_context = FunctionContext {
            return_type_group,
            return_type: return_type.cloned(),
        };
        let mut scope_context = ScopeContext {
            symbol_table: symbol_table.clone(),
            promoted_flow: false,
            branch_returned: false,
            level: 0,
        };

        let statements = self.translate_statements_safe(statement_nodes, &mut scope_context, &mut function_context)?;

        let return_promoted_value = match return_type_group {
            FunctionReturnTypeGroup::None => {
                return Ok(quote! {
                    let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
                    #statements
                })
            }
            FunctionReturnTypeGroup::Single => {
                let return_type_name = self.get_type_name(&function_context.return_type.clone().ok_or(CodegenError::InvalidReturnType)?)?;
                if return_type_name == "Variable" {
                    quote! { return Ok(lift::types::Variable::new_air(builder.get_block_param(return_block, 0).into(), return_block_param_types[0])); }
                } else if self.records.contains_key(&return_type_name) {
                    quote! {
                        let mut return_args = Vec::new();
                        for i in 0..return_block_param_types.len() {
                            return_args.push(Value::from(builder.get_block_param(return_block, i as u32)));
                        }
                        let (to_return, _) = return_value.pack_from_air_values_and_types(&return_args, &return_block_param_types)?;
                        return Ok(to_return.try_into()?);
                    }
                } else {
                    return Err(CodegenError::NotImplemented(file!(), line!()));
                }
            }
            FunctionReturnTypeGroup::Tuple(n) => {
                let inner_type_names =
                    self.get_tuple_inner_type_names(&function_context.return_type.clone().ok_or(CodegenError::InvalidReturnType)?)?;
                if inner_type_names.iter().all(|s| s == "Variable") {
                    let mut tuple_entries = Vec::new();
                    for i in 0..n {
                        let index_lit_int = LitInt::new(&i.to_string(), Span::call_site());
                        tuple_entries.push(quote! { lift::types::Variable::new_air(builder.get_block_param(return_block, #index_lit_int).into(), return_block_param_types[#index_lit_int]) });
                    }
                    quote! { return Ok((#(#tuple_entries),*)); }
                } else {
                    // TODO: handle records in tuples
                    quote! { return Err(AArch64LifterError::NotImplemented(file!(), line!())); }
                }
            }
        };

        Ok(quote! {
            let mut assigns_0: BTreeMap<String, lift::types::AirPackable> = BTreeMap::new();
            let mut has_promoted_returns = false;
            let mut return_block_preds = BTreeMap::<BasicBlock, Vec<Value>>::new();
            let mut return_block_param_types = Vec::new();
            let mut return_value: lift::types::AirPackable = lift::types::Variable::from(common::types::boolean::FALSE).into();
            #statements
            if has_promoted_returns {
                let return_block = sequencer.get_block(pc.to_bits()?.value as u64, lift::types::BlockType::IntraBlock, builder, &return_block_param_types)?;
                for (pred_block, args) in return_block_preds {
                    builder.set_insert_block(pred_block);
                    builder.jump(return_block, args);
                }
                builder.set_insert_block(return_block);
                #return_promoted_value
            }
        })
    }

    fn get_implicit_params(
        &mut self,
        params: &Vec<TypedIdentifier>,
        opt_return_type: Option<&AstNode>,
        symbol_table: &mut BTreeMap<String, VariableMetadata>,
    ) -> Result<Vec<TokenStream>, CodegenError> {
        let mut param_names = params
            .iter()
            .map(|param| AstAnalyzer::normalize_ident(&param.ident))
            .collect::<Vec<String>>();
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
                        // type_name: "Variable".to_string(),
                        air_assigned: false,
                    },
                );
                quote! { #param_ident: lift::types::Variable }
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
            NodeSubtype::App => {
                let data = unwrap_node_data!(return_type, NodeData::TypeApp);

                let implicit_params = data
                    .exprs
                    .iter()
                    .map(|expr| self.get_implicit_params_from_expression(expr))
                    .collect::<Result<Vec<Vec<String>>, CodegenError>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<String>>();

                Ok(implicit_params)
            }
            NodeSubtype::OfExpr => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Register => Ok(vec![]),
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

    fn check_return_type_enum(&self, node: &AstNode) -> Result<FunctionReturnTypeGroup, CodegenError> {
        match node.node_subtype {
            NodeSubtype::Constructor => Ok(FunctionReturnTypeGroup::Single),
            NodeSubtype::Bits | NodeSubtype::App | NodeSubtype::OfExpr | NodeSubtype::Register => Ok(FunctionReturnTypeGroup::Single),
            NodeSubtype::Array => Err(CodegenError::NotImplemented(file!(), line!())),
            NodeSubtype::Tuple => {
                let data = unwrap_node_data!(node, NodeData::TypeTuple);

                Ok(FunctionReturnTypeGroup::Tuple(data.tys.len()))
            }
            _ => Err(CodegenError::InvalidNodeType(node.node_type, node.node_subtype)),
        }
    }
}
