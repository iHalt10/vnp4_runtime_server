use super::error::GenerateTargetConfigProcessError;
use crate::target::driver::XilVitisNetP4TargetConfig;
use crate::target::program::Program;
use crate::target::schema::TargetConfigSchema;
use libloading::Library;
use libloading::Symbol;
use std::path::PathBuf;

pub struct GenerateTargetConfigProcess {
    library_file: PathBuf,
    program_file: PathBuf,
    target_name: String,
}

impl GenerateTargetConfigProcess {
    pub fn new(library_file: PathBuf, program_file: PathBuf, target_name: String) -> Self {
        Self {
            library_file,
            program_file,
            target_name,
        }
    }

    pub fn execute(&self) -> Result<(), GenerateTargetConfigProcessError> {
        unsafe {
            let program = Program::load_json(self.program_file.as_path())?;
            let lib = Library::new(self.library_file.as_path())?;
            let symbol: Symbol<*mut XilVitisNetP4TargetConfig> = lib.get(&self.get_symbol_name())?;
            let config = *symbol;
            let schema = TargetConfigSchema::from_driver_config(*config, program);
            schema.save_json("target-config.json")?;
        }

        return Ok(());
    }

    fn get_symbol_name(&self) -> Vec<u8> {
        format!("XilVitisNetP4TargetConfig_{}", self.target_name).into_bytes()
    }
}
