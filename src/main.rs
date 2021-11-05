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


pub fn print_hello_world() -> () {
    println!("{}", "hello world!");
}

fn main() {
    let mut included_functions = IncludedFunctions::new();
    included_functions.add_print_int_function();
    included_functions.add_function("print_hello_world", print_hello_world as *const u8);

    // Define the compiler util
    let mut compiler_util = CompilerUtil::new(Some(included_functions));

    // Create a new function
    compiler_util.new_function(None);

    // Create a struct to create the function
    let mut builder = FunctionBuilder::new(&mut compiler_util.jit.ctx.func, &mut compiler_util.jit.builder_context);
    let mut func_creator = FunctionCreator::new(&mut builder, &mut compiler_util.jit.module);


    // Call function print hello world
    {
        let int = func_creator.module.target_config().pointer_type();
        let mut sig = func_creator.module.make_signature();
            sig.params.push(AbiParam::new(int));
            sig.returns.push(AbiParam::new(int));

        func_creator.create_variable("some_var_name", Some(int));
        let val = func_creator.get_value_from_variable("some_var_name");
        func_creator.call_function("print_hello_world", sig, &[val]);
    }

    // Create a variable x inside function
    func_creator.create_variable("x", None);

    // Assign x to 10
    let val = func_creator.get_value_from_int(10);
    func_creator.assign_value("x", val);

    // Print 10
    func_creator.print_int(val);

    // Add a with 5 and store result in Val3
    let val1 = func_creator.get_value_from_variable("x");
    let val2 = func_creator.get_value_from_int(5);
    let val3 = func_creator.add_int_values(val1, val2);

    // Finalize function to return Val3
    func_creator.finalize_func(val3);

    // End the function and get its id
    let id = compiler_util.end_function("My function".to_string());

    // End the entire program and run the function created
    compiler_util.end_program();
    let result = compiler_util.run_code(id);
    println!("{}", result);

    //compile_code("x = 7 ; x += 8 + 10 * 5 + 3 ; return_var = x ; x = 3".to_string());
    //println!("Program finished!");
}