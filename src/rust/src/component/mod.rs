mod lua_engine;
mod lua_globals;
mod lua_packages;
mod lua_utils;
mod lua_functions;

use std::sync::{Arc, Mutex};
use addin1c::{name, Variant};
use serde_json::json;
use crate::core::getset;
use lua_engine::LuaEngine;

pub const METHODS: &[&[u16]] = &[
    name!("ExecuteString"),       // 0 - Выполнить Lua код из строки
    name!("ExecuteFile"),         // 1 - Выполнить Lua код из файла
    name!("ExecuteBytecode"),     // 2 - Выполнить предкомпилированный байт-код
    name!("ExecuteBytecodeFile"), // 3 - Выполнить предкомпилированный байт-код
    name!("CompileToBytecode"),   // 4 - Компилировать код в байт-код
    name!("CallFunction"),        // 5 - Вызвать функцию Lua
    name!("SetGlobal"),           // 6 - Установить глобальную переменную
    name!("GetGlobal"),           // 7 - Получить глобальную переменную
    name!("AddPackage"),          // 8 - Добавить пакет Lua
    name!("LoadPackageFromFile"), // 9 - Загрузить пакет из файла
    name!("GetPackages"),         // 10 - Получить список пакетов
    name!("Reset"),               // 11 - Сбросить состояние Lua
];

pub fn get_params_amount(num: usize) -> usize {
    match num {
        0 => 1,  // ExecuteString - код
        1 => 1,  // ExecuteFile - путь к файлу
        2 => 1,  // ExecuteBytecode - байт-код
        3 => 1,  // ExecuteBytecodeFile - путь к файлу
        4 => 1,  // CompileToBytecode - код
        5 => 2,  // CallFunction - имя функции, аргументы (JSON)
        6 => 2,  // SetGlobal - имя переменной, значение (JSON)
        7 => 1,  // GetGlobal - имя переменной
        8 => 2,  // AddPackage - имя пакета, код
        9 => 2,  // LoadPackageFromFile - имя пакета, путь к файлу
        10 => 0,  // GetPackages - без параметров
        11 => 0, // Reset - без параметров
        _ => 0,
    }
}

pub fn cal_func(obj: &mut AddIn, num: usize, params: &mut [Variant]) -> Box<dyn getset::ValueType> {

    let mut engine = match obj.lua_engine.lock() {
        Ok(engine) => engine,
        Err(_) => return Box::new(format_json_error("Failed to lock Lua engine")),
    };

    match num {
        0 => { // ExecuteString
            if params.is_empty() {
                return Box::new(format_json_error("Missing code parameter"));
            }
            let code = params[0].get_string().unwrap_or_default();
            match engine.execute_string(&code) {
                Ok(result) => Box::new(json!({"result": true, "data": result}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        1 => { // ExecuteFile
            if params.is_empty() {
                return Box::new(format_json_error("Missing file path parameter"));
            }
            let path = params[0].get_string().unwrap_or_default();
            match engine.execute_file(&path) {
                Ok(result) => Box::new(json!({"result": true, "data": result}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        2 => { // ExecuteBytecode
            if params.is_empty() {
                return Box::new(format_json_error("Missing bytecode parameter"));
            }
            let bytecode = params[0].get_blob().unwrap_or(&[]).to_vec();
            match engine.execute_bytecode(&bytecode) {
                Ok(result) => Box::new(json!({"result": true, "data": result}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        3 => { // ExecuteBytecodeFile
            if params.is_empty() {
                return Box::new(format_json_error("Missing file path parameter"));
            }
            let path = params[0].get_string().unwrap_or_default();
            match engine.execute_bytecode_file(&path) {
                Ok(result) => Box::new(json!({"result": true, "data": result}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        }
        4 => { // CompileToBytecode
            if params.is_empty() {
                return Box::new(format_json_error("Missing code parameter"));
            }
            let code = params[0].get_string().unwrap_or_default();
            match engine.compile_to_bytecode(&code) {
                Ok(bytecode) => {
                    let mut result = Vec::<u8>::new();
                    result.extend_from_slice(&bytecode);
                    Box::new(result)
                },
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        5 => { // CallFunction
            if params.len() < 2 {
                return Box::new(format_json_error("Missing function name or arguments"));
            }
            let func_name = params[0].get_string().unwrap_or_default();
            let args_json = params[1].get_string().unwrap_or_default();
            
            let args: Vec<serde_json::Value> = match serde_json::from_str(&args_json) {
                Ok(args) => args,
                Err(e) => return Box::new(format_json_error(&format!("Invalid JSON arguments: {}", e))),
            };
            
            match engine.call_function(&func_name, args) {
                Ok(result) => Box::new(json!({"result": true, "data": result}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        6 => { // SetGlobal
            if params.len() < 2 {
                return Box::new(format_json_error("Missing variable name or value"));
            }
            let var_name = params[0].get_string().unwrap_or_default();
            let value_json = params[1].get_string().unwrap_or_default();

            let parsed_value: serde_json::Value = match serde_json::from_str(&value_json) {
                Ok(value) => value,
                Err(e) => return Box::new(format_json_error(&format!("Invalid JSON value: {}", e))),
            };

            let value = match &parsed_value {
                serde_json::Value::Object(obj) => {
                    match obj.get("data") {
                        Some(data) => data.clone(),
                        None => return Box::new(format_json_error("Missing 'data' key in value object")),
                    }
                },
                _ => return Box::new(format_json_error("Value must be an object with 'data' key")),
            };
            
            match engine.set_global(&var_name, value) {
                Ok(_) => Box::new(json!({"result": true}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        7 => { // GetGlobal
            if params.is_empty() {
                return Box::new(format_json_error("Missing variable name"));
            }
            let var_name = params[0].get_string().unwrap_or_default();
            match engine.get_global(&var_name) {
                Ok(result) => Box::new(json!({"result": true, "data": result}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        8 => { // AddPackage
            if params.len() < 2 {
                return Box::new(format_json_error("Missing package name or code"));
            }
            let package_name = params[0].get_string().unwrap_or_default();
            let code = params[1].get_string().unwrap_or_default();
            
            match engine.add_package(package_name, code) {
                Ok(_) => Box::new(json!({"result": true}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        9 => { // LoadPackageFromFile
            if params.len() < 2 {
                return Box::new(format_json_error("Missing package name or file path"));
            }
            let package_name = params[0].get_string().unwrap_or_default();
            let file_path = params[1].get_string().unwrap_or_default();
            
            match engine.load_package_from_file(package_name, &file_path) {
                Ok(_) => Box::new(json!({"result": true}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        10 => { // GetPackages
            let packages = engine.get_packages();
            Box::new(json!({"result": true, "data": packages}).to_string())
        },
        11 => { // Reset
            match engine.reset() {
                Ok(_) => Box::new(json!({"result": true}).to_string()),
                Err(e) => Box::new(format_json_error(&e.to_string())),
            }
        },
        _ => Box::new(format_json_error("Unknown method"))
    }
}

pub const PROPS: &[&[u16]] = &[];

pub struct AddIn {
    lua_engine: Arc<Mutex<LuaEngine>>
}

impl AddIn {
    pub fn new() -> Self {
        let engine = LuaEngine::new()
            .unwrap_or_else(|e| panic!("LuaEngine init failed: {}", e));
        AddIn {
            lua_engine: Arc::new(Mutex::new(engine))
        }
    }

    pub fn get_field_ptr(&self, index: usize) -> *const dyn getset::ValueType {
        match index {
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn get_field_ptr_mut(&mut self, index: usize) -> *mut dyn getset::ValueType {
        self.get_field_ptr(index) as *mut _
    }
}

pub fn format_json_error(error: &str) -> String {
    json!({"result": false, "error": error}).to_string()
}

impl Drop for AddIn {
    fn drop(&mut self) {
    }
}