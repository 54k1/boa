//! Try..catch node execution.

use super::{Executable, Interpreter};
use crate::{
    builtins::value::Value,
    environment::lexical_environment::{new_declarative_environment, VariableScope},
    syntax::ast::node::Try,
    BoaProfiler, Result,
};

#[cfg(test)]
mod tests;

impl Executable for Try {
    fn run(&self, interpreter: &mut Interpreter) -> Result<Value> {
        let _timer = BoaProfiler::global().start_event("Try", "exec");
        let res = self.block().run(interpreter).map_or_else(
            |err| {
                if let Some(catch) = self.catch() {
                    {
                        let env = &mut interpreter.realm_mut().environment;
                        env.push(new_declarative_environment(Some(
                            env.get_current_environment_ref().clone(),
                        )));

                        if let Some(param) = catch.parameter() {
                            env.create_mutable_binding(
                                param.to_owned(),
                                false,
                                VariableScope::Block,
                            )
                            .map_err(|e| e.to_error(interpreter))?;

                            let env = &mut interpreter.realm_mut().environment;
                            env.initialize_binding(param, err)
                                .map_err(|e| e.to_error(interpreter))?;
                        }
                    }

                    let res = catch.block().run(interpreter);

                    // pop the block env
                    let _ = interpreter.realm_mut().environment.pop();

                    res
                } else {
                    Err(err)
                }
            },
            Ok,
        );

        if let Some(finally) = self.finally() {
            finally.run(interpreter)?;
        }

        res
    }
}
