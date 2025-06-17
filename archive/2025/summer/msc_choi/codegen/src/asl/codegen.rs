use crate::asl::ast::{AstAnalyzer, InstructionSet, NodeSubtype, NodeType};
use crate::asl::decode_transpiler::DecodeTranspiler;
use crate::asl::lift_transpiler::LiftTranspiler;
use crate::CodeGenerator;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use syn::parse_file;
use thiserror::Error;

pub struct AslCodeGenerator {
    analyzer: Rc<AstAnalyzer>,

    common_types: TokenStream,
    common_output_path: String,

    decode_transpiler: DecodeTranspiler,
    decode_logic: TokenStream,
    decode_blocks: BTreeMap<String, TokenStream>,
    decode_helpers: TokenStream,
    decode_output_path: String,

    lift_transpiler: LiftTranspiler,
    lift_logic: TokenStream,
    lift_blocks: BTreeMap<String, TokenStream>,
    lift_helpers: TokenStream,
    lift_types: TokenStream,
    lift_output_path: String,
}

impl CodeGenerator for AslCodeGenerator {
    type E = CodegenError;

    fn new(
        ast_file_path: &str,
        decode_module_dir_path: &str,
        lift_module_path: &str,
        common_module_path: &str,
        supported_instructions: BTreeSet<String>,
    ) -> Result<Self, CodegenError> {
        let analyzer = Rc::new(AstAnalyzer::new(ast_file_path, supported_instructions.clone())?);

        let decode_transpiler = DecodeTranspiler::new(
            analyzer.clone(),
            &format!("{}/helpers.rs", decode_module_dir_path),
            supported_instructions.clone(),
        )?;
        let lift_transpiler = LiftTranspiler::new(
            analyzer.clone(),
            &format!("{}/helpers.rs", lift_module_path),
            supported_instructions.clone(),
        )?;

        Ok(Self {
            analyzer,

            common_types: TokenStream::new(),
            common_output_path: format!("{}/generated", common_module_path),

            decode_transpiler,
            decode_logic: TokenStream::new(),
            decode_blocks: BTreeMap::new(),
            decode_helpers: TokenStream::new(),
            decode_output_path: format!("{}/generated", decode_module_dir_path),

            lift_transpiler,
            lift_logic: TokenStream::new(),
            lift_blocks: BTreeMap::new(),
            lift_helpers: TokenStream::new(),
            lift_types: TokenStream::new(),
            lift_output_path: format!("{}/generated", lift_module_path),
        })
    }

    fn generate_shared_code(&mut self) -> Result<(), CodegenError> {
        let mut instruction_variants = quote! { NOP, UNPRED, UNALLOC, UNDEF, };
        let mut operands_structs = TokenStream::new();

        for (opcode, operands) in self.analyzer.get_opcode_and_operands(InstructionSet::A64)? {
            let opcode_ident = Ident::new(opcode, Span::call_site());
            let operands_ident = Ident::new(format!("{}_operands", opcode).as_str(), Span::call_site());
            instruction_variants.extend(quote! {
                #opcode_ident(Box<#operands_ident>),
            });

            let operand_fields = operands
                .iter()
                .map(|metadata| {
                    let operand_name_ident = Ident::new(&metadata.name, Span::call_site());
                    let operand_type_ident = Ident::new(&metadata.type_name, Span::call_site());
                    quote! {
                        pub #operand_name_ident: common::types::#operand_type_ident,
                    }
                })
                .collect::<Vec<TokenStream>>();

            operands_structs.extend(quote! {
                #[derive(Debug)]
                pub struct #operands_ident {
                    #(#operand_fields)*
                }
            });
        }

        let translated_enums = self.decode_transpiler.translate_enums()?;

        let translated_constants = self.decode_transpiler.translate_constants()?;

        self.common_types.extend(quote! {
            #![allow(nonstandard_style, unused)]
            use crate::arm64::AArch64LifterError;
            use crate::arm64::{common, decode};
            use crate::arm64::common::types::BigIntExt;
            use crate::integer_to_usize;
            use num_bigint::BigInt;
            use num_traits::ToPrimitive;
            use once_cell::sync::Lazy;
            use strum_macros::EnumCount;
            #[derive(Debug)]
            pub enum Instruction {
                #instruction_variants
            }
            #operands_structs
            #translated_enums
            #translated_constants
        });

        Ok(())
    }

    fn generate_decode_logic(&mut self) -> Result<(), CodegenError> {
        let decode_inner_logic = self.decode_transpiler.translate_decode_logic()?;

        self.decode_logic.extend(quote! {
            #![allow(nonstandard_style, unused)]
            use crate::arm64::AArch64LifterError;
            use crate::arm64::{decode, common};
            use crate::integer_to_usize;
            use num_bigint::BigInt;
            use num_traits::ToPrimitive;
            use std::collections::BTreeMap;
            pub fn generated_decode_logic(reader: &mut decode::BitReader) -> Result<common::types::Instruction, AArch64LifterError> {
                let mut instruction: common::types::Instruction;
                #decode_inner_logic
                Ok(instruction)
            }
        });

        let decode_blocks = self.decode_transpiler.get_decode_blocks();
        for (encoding_name, code) in decode_blocks {
            self.decode_blocks.insert(
                encoding_name.to_string(),
                quote! {
                    #![allow(nonstandard_style, unused)]
                    use crate::arm64::AArch64LifterError;
                    use crate::arm64::{decode, common};
                    use crate::arm64::common::types::BigIntExt;
                    use crate::integer_to_usize;
                    use num_bigint::BigInt;
                    use num_traits::{One, ToPrimitive};
                    use std::collections::BTreeMap;
                    #code
                },
            );
        }

        self.decode_helpers.extend(quote! {
            #![allow(nonstandard_style, unused)]
            use crate::arm64::AArch64LifterError;
            use crate::arm64::{decode, common};
            use crate::arm64::common::types::BigIntExt;
            use crate::integer_to_usize;
            use num_bigint::BigInt;
            use num_traits::{One, ToPrimitive};
        });

        let helpers = self.decode_transpiler.get_helpers();
        for helper in helpers {
            self.decode_helpers.extend(helper);
        }

        Ok(())
    }

    fn generate_lift_logic(&mut self) -> Result<(), CodegenError> {
        let execute_inner_logic = self.lift_transpiler.translate_lift_logic()?;

        self.lift_logic.extend(quote! {
            #![allow(nonstandard_style, unused)]
            use crate::arm64::AArch64LifterError;
            use crate::arm64::{lift, common};
            use crate::arm64::lift::types::BlockSequencer;
            use tnj::air::instructions::builder::InstructionBuilder;
            pub fn generated_lift_logic(instruction: common::types::Instruction, builder: &mut InstructionBuilder, sequencer: &mut BlockSequencer, pc: u64) -> Result<(), AArch64LifterError> {
                match instruction {
                    #execute_inner_logic
                };
                Ok(())
            }
        });

        let lift_blocks = self.lift_transpiler.get_lift_blocks();
        for (encoding_name, code) in lift_blocks {
            self.lift_blocks.insert(
                encoding_name.to_string(),
                quote! {
                    #![allow(nonstandard_style, unused)]
                    use crate::arm64::AArch64LifterError;
                    use crate::arm64::{lift, common};
                    use crate::arm64::common::types::BigIntExt;
                    use crate::{integer_to_usize, integer_to_u32, integer_to_u64};
                    use crate::arm64::lift::types::{AirPackable, BlockSequencer};
                    use tnj::air::instructions::builder::InstructionBuilder;
                    use tnj::air::instructions::Value;
                    use tnj::arch::reg::Reg;
                    use tnj::types::Type;
                    use num_bigint::BigInt;
                    use num_traits::{One, ToPrimitive};
                    use std::collections::BTreeMap;
                    #code
                },
            );
        }

        self.lift_helpers.extend(quote! {
            #![allow(nonstandard_style, unused)]
            use crate::arm64::AArch64LifterError;
            use crate::arm64::{lift, common};
            use crate::arm64::common::types::BigIntExt;
            use crate::{integer_to_usize, integer_to_u64};
            use crate::arm64::lift::types::{AirPackable, BlockSequencer};
            use tnj::air::instructions::builder::InstructionBuilder;
            use tnj::air::instructions::{BasicBlock, Value};
            use tnj::types::Type;
            use num_bigint::BigInt;
            use num_traits::{One, ToPrimitive, Zero};
            use std::collections::BTreeMap;
        });

        let helpers = self.lift_transpiler.get_helpers();
        for helper in helpers {
            self.lift_helpers.extend(helper);
        }

        let mut variable_variants = TokenStream::new();
        let mut from_implements = TokenStream::new();
        let mut to_implements = TokenStream::new();
        let mut promote_branches = TokenStream::new();
        for (enum_type, _) in self.analyzer.get_enums()? {
            let enum_type_ident = Ident::new(enum_type, Span::call_site());

            variable_variants.extend(quote! {
                #enum_type_ident(common::types::#enum_type_ident),
            });

            from_implements.extend(quote! {
                impl From<common::types::#enum_type_ident> for lift::types::Variable {
                    fn from(value: common::types::#enum_type_ident) -> Self {
                        lift::types::Variable::Rust(RustVariable::#enum_type_ident(value))
                    }
                }
            });

            let to_enum_type_ident = Ident::new(format!("to_{}", enum_type).as_str(), Span::call_site());
            to_implements.extend(quote! {
                pub fn #to_enum_type_ident(&self) -> Result<common::types::#enum_type_ident, AArch64LifterError> {
                    match self {
                        lift::types::Variable::Rust(RustVariable::#enum_type_ident(n)) => Ok(n.clone()),
                        _ => Err(AArch64LifterError::VariableNotExpectedEnum),
                    }
                }
            });

            promote_branches.extend(quote! {
                lift::types::Variable::Rust(RustVariable::#enum_type_ident(n)) => Ok(Self::air_from_enum(builder, *n as u8)?),
            });
        }

        let translated_records = self.lift_transpiler.translate_records()?;

        let sequencer_logic = self.lift_transpiler.translate_sequencer_logic()?;

        self.lift_types.extend(quote! {
            #![allow(nonstandard_style, unused)]
            use crate::arm64::{common, decode, lift, AArch64LifterError};
            use crate::arm64::lift::types::BlockSequencer;
            use std::cmp::Reverse;
            use std::collections::{BinaryHeap, BTreeMap};
            use std::convert::TryFrom;
            use std::hash::Hash;
            use target_lexicon::{Aarch64Architecture, Architecture};
            use tnj::air::instructions::{BasicBlock, CodeRegion, Value};
            use tnj::air::instructions::builder::InstructionBuilder;
            use tnj::types::Type;
            use tnj::arch::get_arch;
            use num_traits::ToPrimitive;
            #[derive(Debug, Clone, Eq, PartialEq)]
            pub enum RustVariable {
                integer(common::types::integer),
                bits(common::types::bits),
                #variable_variants
            }
            #from_implements
            impl lift::types::Variable {
                pub fn promote_to_air(&self, builder: &mut InstructionBuilder) -> Result<Self, AArch64LifterError> {
                    match self {
                        lift::types::Variable::Rust(RustVariable::integer(n)) => Ok(Self::air_from_integer(builder, n.clone())?),
                        lift::types::Variable::Rust(RustVariable::bits(n)) => Ok(Self::air_from_bits(builder, *n)?),
                        lift::types::Variable::Rust(RustVariable::boolean(n)) => Ok(Self::air_from_boolean(builder, *n)?),
                        #promote_branches
                        lift::types::Variable::Air(_) => Ok(self.clone()),
                        _ => Err(AArch64LifterError::NotImplemented(file!(), line!())),
                    }
                }
                #to_implements
            }
            #translated_records
            #sequencer_logic
        });

        Ok(())
    }

    fn write_to_files(&self) -> Result<(), CodegenError> {
        let decode_dir_path = Path::new(&self.decode_output_path);
        Self::create_directories(decode_dir_path)?;

        let lift_dir_path = Path::new(&self.lift_output_path);
        Self::create_directories(lift_dir_path)?;

        let common_dir_path = Path::new(&self.common_output_path);
        Self::create_directories(common_dir_path)?;

        Self::write_to_file(decode_dir_path.join("decode_logic.rs"), &self.decode_logic)?;
        Self::write_to_file(decode_dir_path.join("helpers.rs"), &self.decode_helpers)?;
        let mut decode_blocks_mod = TokenStream::new();
        for (encoding_name, code) in self.decode_blocks.iter() {
            Self::write_to_file(decode_dir_path.join(format!("decode_blocks/{}.rs", encoding_name)), code)?;
            let encoding_name_ident = Ident::new(&encoding_name, Span::call_site());
            decode_blocks_mod.extend(quote! {
                mod #encoding_name_ident;
                pub use #encoding_name_ident::*;
            });
        }
        Self::write_to_file(decode_dir_path.join("decode_blocks/mod.rs"), &decode_blocks_mod)?;
        Self::write_to_file(
            decode_dir_path.join("mod.rs"),
            &quote! {
                pub mod decode_blocks;
                pub mod decode_logic;
                pub mod helpers;
            },
        )?;

        Self::write_to_file(lift_dir_path.join("lift_logic.rs"), &self.lift_logic)?;
        Self::write_to_file(lift_dir_path.join("helpers.rs"), &self.lift_helpers)?;
        Self::write_to_file(lift_dir_path.join("types.rs"), &self.lift_types)?;
        let mut lift_blocks_mod = TokenStream::new();
        for (instruction_name, code) in self.lift_blocks.iter() {
            Self::write_to_file(lift_dir_path.join(format!("lift_blocks/{}.rs", instruction_name)), code)?;
            let instruction_name_ident = Ident::new(&instruction_name, Span::call_site());
            lift_blocks_mod.extend(quote! {
                mod #instruction_name_ident;
                pub use #instruction_name_ident::*;
            });
        }
        Self::write_to_file(lift_dir_path.join("lift_blocks/mod.rs"), &lift_blocks_mod)?;
        Self::write_to_file(
            lift_dir_path.join("mod.rs"),
            &quote! {
                pub mod lift_blocks;
                pub mod lift_logic;
                pub mod helpers;
                pub mod types;
            },
        )?;

        Self::write_to_file(common_dir_path.join("types.rs"), &self.common_types)?;
        Self::write_to_file(
            common_dir_path.join("mod.rs"),
            &quote! {
                pub mod types;
            },
        )?;

        Ok(())
    }
}

impl AslCodeGenerator {
    fn create_directories(path: &Path) -> Result<(), CodegenError> {
        if !path.parent().map(|p| p.exists()).unwrap_or(false) {
            return Err(CodegenError::MissingDirectory(
                path.parent().map(|p| p.display().to_string()).unwrap_or_else(|| "Unknown".to_string()),
            ));
        }

        Ok(())
    }

    fn write_to_file(path: PathBuf, code: &TokenStream) -> Result<(), CodegenError> {
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| CodegenError::IoError(format!("Failed to create directory {:?}: {}", parent, e)))?;
            }
        }

        let formatted_code = match parse_file(&code.to_string()) {
            Ok(syntax_tree) => prettyplease::unparse(&syntax_tree),
            Err(_) => code.to_string(),
        };

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|e| CodegenError::IoError(format!("Failed to open file {:?}: {}", path, e)))?;

        println!("{:?}", path);

        write!(file, "{}", formatted_code).map_err(|e| CodegenError::IoError(format!("Failed to write to file: {}", e)))?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Necessary nodes are missing in the AST")]
    MissingAstNodes,

    #[error("No opcode analysis found for {0}")]
    MissingOpcodeAnalysis(InstructionSet),

    #[error("No operand analysis for {0}")]
    MissingOperandAnalysis(InstructionSet),

    #[error("No operand analysis for instruction {0}")]
    MissingOperandAnalysisForInstruction(String),

    #[error("No decoder definition found for {0}")]
    MissingDecoderDef(InstructionSet),

    #[error("Unknown instruction set {0}")]
    UnknownInstructionSet(String),

    #[error("Mismatched node type: expected one of {0}, but found node_type {1:?} and node_subtype {2:?}")]
    MismatchedNodeType(String, NodeType, NodeSubtype),

    #[error("Encountered unexpected node_type {0:?} and node_subtype {1:?} during translation")]
    UnexpectedNodeType(NodeType, NodeSubtype),

    #[error("Encountered invalid combination of node_type {0:?} and node_subtype {1:?} during translation")]
    InvalidNodeType(NodeType, NodeSubtype),

    #[error("Encountered type {0} during translation")]
    UnhandledType(String),

    #[error("Missing node_data for node_type {0:?} and node_subtype {1:?}")]
    MissingNodeData(NodeType, NodeSubtype),

    #[error("BuiltinFunction {0} should be manually implemented")]
    MissingBuiltinFunctionImplementation(String),

    #[error("Wrong number of implicit parameters for helper function {0}: ASL {1}, airlift {2}")]
    WrongImplicitParamCount(String, usize, usize),

    #[error("Helper {0} not found in helper table")]
    MissingHelperInHelperTable(String),

    #[error("Variable {0} not found in symbol table")]
    MissingVariableInSymbolTable(String),

    #[error("Operand variant for variable {0} not found in symbol table metadata")]
    MissingOperandVariantInSymbolTable(String),

    #[error("Register read syntax not recognized")]
    UnrecognizedRegisterReadSyntax,

    #[error("Register write syntax not recognized")]
    UnrecognizedRegisterWriteSyntax,

    #[error("Flag read syntax not recognized")]
    UnrecognizedFlagReadSyntax,

    #[error("Flag write syntax not recognized")]
    UnrecognizedFlagWriteSyntax,

    #[error("Directory does not exist: {0}")]
    MissingDirectory(String),

    #[error("Function return type incorrectly categorized")]
    InvalidReturnType,

    #[error("Not implemented at {0}:{1}")]
    NotImplemented(&'static str, u32),

    #[error("{0}")]
    IoError(String),

    #[error("{0}")]
    CustomError(String),

    /// ASL validation errors

    #[error("")]
    InvalidDecodeSlice,

    #[error("")]
    InvalidDecodePattern,

    #[error("")]
    InvalidDecodeCaseOrder,

    #[error("Instruction not found for encoding name {0}")]
    EncodingNotFound(String),

    #[error("Instruction not found for instruction name {0}")]
    InstructionNotFound(String),

    #[error("Function {0} not found in the AST")]
    FunctionNotFound(String),

    #[error("Procedure {0} not found in the AST")]
    ProcedureNotFound(String),

    #[error("Enum type {0} has no variants")]
    EnumWithEmptyVariants(String),
}
