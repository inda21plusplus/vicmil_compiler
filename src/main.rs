pub mod jit;
pub use jit::*;
pub mod util;
pub use util::*;

use cranelift::codegen::CodegenError;
use cranelift::codegen::verifier::{VerifierErrors};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, FuncId, Linkage, Module, ModuleError};
use std::collections::HashMap;
use core::mem;

fn main() {
    let mut compiler_util = CompilerUtil::new(None);
    compiler_util.new_function(None);
    let mut builder = FunctionBuilder::new(&mut compiler_util.jit.ctx.func, &mut compiler_util.jit.builder_context);
    let mut func_creator = FunctionCreator::new(&mut builder, &mut compiler_util.jit.module);
    func_creator.create_variable("x");
    let val = func_creator.get_value_from_int(10);
    func_creator.assign_value("x", val);
    let val1 = func_creator.get_value_from_variable("x");
    let val2 = func_creator.get_value_from_int(5);
    let val3 = func_creator.add_int_values(val1, val2);
    func_creator.finalize_func(val3);
    let id = compiler_util.end_function("My function".to_string());
    compiler_util.end_program();
    let result = compiler_util.run_code(id);
    println!("{}", result);

    //compile_code("x = 7 ; x += 8 + 10 * 5 + 3 ; return_var = x ; x = 3".to_string());
    //println!("Program finished!");
}