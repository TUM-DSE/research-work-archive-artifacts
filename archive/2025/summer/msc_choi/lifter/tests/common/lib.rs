use super::simple_variable_map::SimpleVariableMap;
use filecheck::{CheckerBuilder, Value};
use lifter::arm64::AArch64Lifter;
use lifter::Lifter;
use std::time::Instant;

const VARIABLES: [(&str, Value); 2] = [
    ("VAR_NAME", Value::Regex(std::borrow::Cow::Borrowed("v[0-9]+"))),
    ("LABEL", Value::Regex(std::borrow::Cow::Borrowed("[a-zA-Z0-9_]+"))),
];

pub struct CheckInstructionArgs {
    pub variable_map: SimpleVariableMap,
    pub print_to_std: bool,
    pub debug: bool,
}

impl CheckInstructionArgs {
    pub fn new(variable_map: SimpleVariableMap, print_to_std: bool, debug: bool) -> Self {
        Self {
            variable_map,
            print_to_std,
            debug,
        }
    }
}

impl Default for CheckInstructionArgs {
    fn default() -> Self {
        Self::new(SimpleVariableMap::default(), true, false)
    }
}

pub fn check_instruction(bytes: &[u8], proofs: Option<&[u8]>, directives: &str, args: CheckInstructionArgs) -> bool {
    let lifter = AArch64Lifter;
    let start = Instant::now();
    let code_region = lifter.lift(bytes, proofs.unwrap_or(&[]), false).unwrap();
    let duration = start.elapsed();
    let result = code_region.display().to_string();
    if args.debug {
        let blocks_count = code_region.blocks().count();
        let inst_count = code_region.blocks().fold(0, |acc, b| acc + b.inst_count());
        println!("Blocks: {}, Instructions: {}", blocks_count, inst_count);
        println!("Lifting time: {:.2?}", duration);
    }
    if args.print_to_std {
        println!("{}", result);
    }

    let mut variable_map = args.variable_map;
    for (var_name, value) in VARIABLES {
        variable_map.insert(var_name.to_string(), value);
    }
    let mut checker_builder = CheckerBuilder::new();
    let checker_builder = checker_builder.text(directives).expect("Failed to create checker builder");
    let checker = checker_builder.finish();

    checker.check(&result, &variable_map).expect("Filecheck failed")
}
