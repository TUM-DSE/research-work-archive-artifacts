#![cfg_attr(rustfmt, rustfmt_skip)]// ⚠️ Automatically generated file, do not edit! ⚠️

use crate::lifter::yaml_tests::run_test_from_yaml;

#[test]
pub fn test_adds_1() {
    run_test_from_yaml("tests/lifter/insts/tests/adds.yaml", "adds_1");
}
#[test]
pub fn test_adds_2() {
    run_test_from_yaml("tests/lifter/insts/tests/adds.yaml", "adds_2");
}
