use std::collections::BTreeSet;

pub mod asl;

pub trait CodeGenerator {
    type E;

    fn new(
        ast_file_path: &str,
        decode_module_dir_path: &str,
        lift_module_path: &str,
        common_module_path: &str,
        supported_instructions: BTreeSet<String>,
    ) -> Result<Self, Self::E>
    where
        Self: Sized;

    fn generate_shared_code(&mut self) -> Result<(), Self::E>;

    fn generate_decode_logic(&mut self) -> Result<(), Self::E>;

    fn generate_lift_logic(&mut self) -> Result<(), Self::E>;

    fn write_to_files(&self) -> Result<(), Self::E>;
}
