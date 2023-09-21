use crate::{
    compiler::Compiler,
    error::Error,
    value::Value,
    vm::{Environment, Vm},
};

pub struct Interpreter {
    vm: Vm,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { vm: Vm::new() }
    }

    pub fn eval(source: &str, env: &Environment) -> Result<Value, Error> {
        let mut interpreter = Interpreter::new();
        interpreter.run(source, env)
    }

    pub fn run(&mut self, source: &str, env: &Environment) -> Result<Value, Error> {
        let compiler = Compiler::new();
        let module = compiler.compile(source)?;

        self.vm.execute(module, env)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let source = r#"1+2+3+4+5+6+7+8+9"#;

        let mut env = Environment::new();

        let ret = Interpreter::eval(source, &env).unwrap();

        assert_eq!(ret, Value::Integer(45));
    }

    #[test]
    fn test_array() {
        let source = r#"["s", 100][1] > 0"#;

        let mut env = Environment::new();

        let ret = Interpreter::eval(source, &env).unwrap();

        assert_eq!(ret, Value::Bool(true));
    }

    #[test]
    fn test_dict() {
        let source = r#""s" in {s: "String", i: 100}"#;

        let mut env = Environment::new();

        let ret = Interpreter::eval(source, &env).unwrap();

        assert_eq!(ret, Value::Bool(true));
    }
}
