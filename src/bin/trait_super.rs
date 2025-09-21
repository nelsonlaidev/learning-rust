#![allow(unused)]

trait Language {
    fn name(&self) -> String;
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait CompiledLanguage: Language + Compiler {
    fn execute(&self, file_path: &str) {
        let name = self.name();
        println!("name: {name}");

        let cmd = self.compile(file_path);
        println!("cmd: {cmd}");
    }
}

struct Rust;

impl Language for Rust {
    fn name(&self) -> String {
        "rust".to_string()
    }
}

impl Compiler for Rust {
    fn compile(&self, file_path: &str) -> String {
        format!("cargo build {file_path}")
    }
}

impl CompiledLanguage for Rust {}

fn main() {
    // Super trait
    let rust = Rust;
    rust.execute("hello.rs");
}
