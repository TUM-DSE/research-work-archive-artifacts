use codegen::asl::AslCodeGenerator;
use codegen::CodeGenerator;
use prettier_please::unparse;
use quote::{format_ident, quote};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use syn::File;

#[derive(Deserialize)]
struct TestFile {
    tests: Vec<TestSpec>,
}

#[derive(Deserialize)]
struct TestSpec {
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=../codegen");
    println!("cargo:rerun-if-changed=src/arm64/decode/helpers.rs");
    println!("cargo:rerun-if-changed=src/arm64/lift/helpers.rs");
    println!("cargo:rerun-if-changed=../external/asl-interpreter/ast.json");

    println!("cargo:rerun-if-changed=tests/lifter/insts/tests");

    generate_lifter()?;
    generate_tests()?;

    Ok(())
}

fn generate_lifter() -> Result<(), Box<dyn Error>> {
    let supported_instructions = [
        "aarch64_integer_arithmetic_add_sub_carry",
        "aarch64_integer_arithmetic_add_sub_shiftedreg",
        "aarch64_integer_arithmetic_add_sub_extendedreg",
        "aarch64_integer_arithmetic_address_pc_rel",
        "aarch64_integer_logical_immediate",
        "aarch64_integer_logical_shiftedreg",
        "aarch64_integer_shift_variable",
        "aarch64_branch_unconditional_immediate",
        "aarch64_branch_conditional_cond",
        "aarch64_integer_bitfield",
        "aarch64_branch_unconditional_register",
        "aarch64_branch_conditional_compare",
        "aarch64_integer_conditional_compare_register",
        "aarch64_integer_arithmetic_cnt",
        "aarch64_integer_conditional_select",
        "aarch64_integer_ins_ext_extract_immediate",
        "aarch64_memory_ordered",
        "aarch64_memory_pair_general_post_idx",
        "aarch64_memory_literal_general",
        "aarch64_memory_single_general_immediate_signed_post_idx",
        "aarch64_memory_single_general_register",
        "aarch64_memory_single_general_immediate_signed_offset_normal",
        "aarch64_integer_ins_ext_insert_movewide",
        "aarch64_integer_arithmetic_mul_uniform_add_sub",
        "aarch64_integer_arithmetic_rbit",
        "aarch64_integer_arithmetic_rev",
        "aarch64_integer_arithmetic_div",
        "aarch64_integer_arithmetic_mul_widening_32_64",
        "aarch64_integer_arithmetic_mul_widening_64_128hi",
        "aarch64_branch_conditional_test",
    ]
    .iter()
    .map(|&s| s.to_string())
    .collect::<BTreeSet<String>>();

    let mut generator = AslCodeGenerator::new(
        "../external/asl-interpreter/ast.json",
        "src/arm64/decode",
        "src/arm64/lift",
        "src/arm64/common",
        supported_instructions,
    )?;

    generator.generate_shared_code()?;
    generator.generate_decode_logic()?;
    generator.generate_lift_logic()?;

    generator.write_to_files()?;

    Ok(())
}

fn generate_tests() -> Result<(), Box<dyn Error>> {
    let tests_dir = Path::new("tests/lifter/insts/tests");
    let out_dir = Path::new("tests/lifter/insts/generated");
    // let out_dir = env::var_os("OUT_DIR").unwrap();

    if !out_dir.exists() {
        fs::create_dir_all(out_dir).unwrap();
    }

    let mut mod_file_content = "#![cfg_attr(rustfmt, rustfmt_skip)]\n// ⚠️ Automatically generated file, do not edit! ⚠️\n\n".to_string();

    for entry in fs::read_dir(tests_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        let yaml_str = fs::read_to_string(&path).unwrap();
        let test_file: TestFile = serde_yaml::from_str(&yaml_str).unwrap();

        let file_stem = path.file_stem().unwrap().to_str().unwrap();
        let mut rust_tests = "#![cfg_attr(rustfmt, rustfmt_skip)]// ⚠️ Automatically generated file, do not edit! ⚠️\n\nuse crate::lifter::yaml_tests::run_test_from_yaml;\n\n".to_string();

        for test in test_file.tests {
            let test_name = &test.name;
            let test_fn_name = format_ident!("test_{}", test.name);
            let test_file = format!("tests/lifter/insts/tests/{}.yaml", file_stem);
            let ts = quote! {
                #[test]
                pub fn #test_fn_name() {
                    run_test_from_yaml(#test_file, #test_name);
                }
            };
            let file: File = syn::parse2(ts).unwrap();
            rust_tests.push_str(&unparse(&file));
        }

        // write the individual rust test file
        fs::write(out_dir.join(format!("{}.rs", file_stem)), rust_tests).unwrap();

        // append mod declaration
        mod_file_content.push_str(&format!("pub mod {};\n", file_stem));
    }

    fs::write(out_dir.join("mod.rs"), mod_file_content).unwrap();

    Ok(())
}
