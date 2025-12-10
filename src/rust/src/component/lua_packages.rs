use std::path::Path;
use crate::component::lua_engine::LuaEngine;

impl LuaEngine {

    pub fn add_package(&mut self, name: String, code: String) -> Result<(), String> {
        match self.lua.load(&code).exec() {
            Ok(_) => {
                self.packages.insert(name, code);
                Ok(())
            },
            Err(e) => Err(format!("Package execution error: {}", e)),
        }
    }

    pub fn load_package_from_file<P: AsRef<Path>>(&mut self, name: String, path: P) -> Result<(), String> {
        let code = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read package file: {}", e))?;
        self.add_package(name, code)
    }

    pub fn get_packages(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }

}