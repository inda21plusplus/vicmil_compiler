use cranelift::codegen::CodegenError;
use cranelift::codegen::verifier::{VerifierError, VerifierErrors};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, FuncId, Linkage, Module, ModuleError};
//use core::num::dec2flt::number::Number;
use std::collections::HashMap;
use std::{result, slice};
use core::mem;

pub enum Token {
    IdentifierToken(IdentifierToken),
    NumberToken(NumberToken),
    OperatorToken(OperatorToken)
}

pub struct IdentifierToken {
    pub text: String
}

pub struct NumberToken {
    pub num: i32
}

pub enum OperatorType {
    Add,
    Sub,
    Eq
}
pub struct OperatorToken {
    pub op_type: OperatorType
}

pub fn is_number(my_str: String) -> Option<NumberToken> {
    if my_str.len() == 0 {
        return None;
    }
    let mut iter_count = 0;
    for i in my_str.chars().into_iter() {
        if iter_count == 0 && i == '-' {
            continue;
        }
        match i {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            }
            _ => {
                return None;
            }
        }
        iter_count += 1;
    }
    return Some(NumberToken{num: my_str.parse::<i32>().unwrap()});
}

pub fn is_identifier(my_str: String) -> Option<IdentifierToken> {
    if my_str.len() == 0 {
        return None;
    }
    let mut iter_count = 0;
    for i in my_str.chars().into_iter() {
        if "123456789".find(i).is_some() {
            if iter_count == 0 {
                return None;
            }
        }
        else if "abcdefghijklmnopqrstuvwxyz_".find(i).is_some() {
        }
        else {
            return None;
        }
        iter_count += 1;
    }
    return Some(IdentifierToken{text: my_str});
}

pub fn is_operator(my_str: String) -> Option<OperatorToken> {
    let op_type: OperatorType;
    match my_str.as_str() {
        "+=" => { op_type = OperatorType::Add; }
        "-=" => { op_type = OperatorType::Sub; }
        "=" => { op_type = OperatorType::Eq; }
        _ => { return None; }
    }
    return Some(OperatorToken{op_type: op_type});
}

/// The basic JIT class.
pub struct JIT {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    ctx: codegen::Context,

    /// The data context, which is to data objects what `ctx` is to functions.
    data_ctx: DataContext,

    /// The module, with the jit backend, which manages the JIT'd
    /// functions.
    module: JITModule,
}

impl Default for JIT {
    fn default() -> Self {
        let mut builder = JITBuilder::new(cranelift_module::default_libcall_names());

        // Register hello world print function.
        let print_addr = print_hello_world as *const u8;
        builder.symbol("print_hello_world", print_addr);

        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }
}

pub fn print_hello_world() -> () {
    println!("{}", "hello world!");
}

pub fn compile_code(my_str: String) {
    // Use the lexer to split up string into tokens
    let mut token_vec: Vec<Token> = vec!();
    let split_str = my_str.split_ascii_whitespace();
    for i in split_str.into_iter() {
        if let Some(token) = is_operator(i.to_string()) {
            println!("operator: {}", i);
            token_vec.push(Token::OperatorToken(token));
        }
        else if let Some(token) = is_number(i.to_string()) {
            println!("number: {}", i);
            token_vec.push(Token::NumberToken(token));
        }
        else if let Some(token) = is_identifier(i.to_string()) {
            println!("identifier: {}", i);
            token_vec.push(Token::IdentifierToken(token));
        }
        else {
            println!("undefined: {}", i);
        }
    }

    // Parse the string, ( TODO generate a tree)
    // add what it should do to cranelift, (TODO: Treverse tree) 

    // Create the jit
    let mut jit = JIT::default();

    // Declare a variable type to use inside it
    let int = jit.module.target_config().pointer_type();
    use cranelift::prelude::types::I32;

    // Add new function to cranelift(i think?)
    jit.ctx.func.signature.returns.push(AbiParam::new(I32));

    // Create the builder to build function for cranelift.
    let mut builder = FunctionBuilder::new(&mut jit.ctx.func, &mut jit.builder_context);

    // Create block, to start putting code in inside function.
    let entry_block = builder.create_block();

    // Since this is the entry block, add block parameters corresponding to
    // the function's parameters.
    builder.append_block_params_for_function_params(entry_block);

    // Tell the builder to put code created by calls in this block.
    builder.switch_to_block(entry_block);

    // And, tell the builder that this block will have no further
    // predecessors. Since it's the entry block, it won't have any
    // predecessors.
    builder.seal_block(entry_block);

    // Declare variable to hold variables in function
    let variables: HashMap<String, Variable> = HashMap::new();

    // Declare return variable
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let var = Variable::new(0);
    let name = "return_var";

    // Declare var in variables for function
    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, I32);
    }

    // Create a struct to keep track of variables used to create the function
    // (translate your own code of your language into cranelift)
    let mut trans = FunctionTranslator {
        int,
        builder,
        variables,
        module: &mut jit.module,
    };

    // Insert the type of input and output of the function, its signature
    let mut sig = trans.module.make_signature();
    sig.params.push(AbiParam::new(int));
    sig.returns.push(AbiParam::new(I32));


    let mut var_iter = 1;
    // Parse the code from the custom language
    if token_vec.len() >= 3 {
        for i in 0..token_vec.len()-2 {
            let token1: &Token = token_vec.get(i).unwrap();
            let token2: &Token = token_vec.get(i+1).unwrap();
            let token3: &Token = token_vec.get(i+2).unwrap();
            if let Token::IdentifierToken(token1_ident) = token1 {
                if let Token::OperatorToken(token2_ident) = token2 {
                    if let Token::IdentifierToken(token3_ident) = token3 {
                        // Make sure the second variable is defined
                        if !trans.variables.contains_key(token3_ident.text.as_str()) {
                            println!("Variable not defined: {}, skipping", token3_ident.text);
                            continue;
                        }

                        // Declare the first variable if it is not already defined
                        if !trans.variables.contains_key(token1_ident.text.as_str()) {
                            let var = Variable::new(var_iter);
                            var_iter += 1;
                            trans.variables.insert(token1_ident.text.as_str().into(), var);
                            trans.builder.declare_var(var, I32);
                        }

                        let var1 = trans.variables.get(token1_ident.text.as_str()).unwrap();
                        let var2 = trans.variables.get(token3_ident.text.as_str()).unwrap();
                        let val2 = trans.builder.use_var(*var2);

                        // Perform operation between the two variables
                        match token2_ident.op_type {
                            OperatorType::Add => {
                                println!("add variables: {} {}", token1_ident.text, token3_ident.text);
                                let val1 = trans.builder.use_var(*var1);
                                // Add variables and store result
                                let result = trans.builder.ins().iadd(val1, val2);
                                // Set var1 = result                                
                                trans.builder.def_var(*var1, result);
                            }
                            OperatorType::Sub => {
                                println!("sub variables: {} {}", token1_ident.text, token3_ident.text);
                                let val1 = trans.builder.use_var(*var1);
                                // Add variables and store result
                                let result = trans.builder.ins().isub(val1, val2);
                                // Set var1 = result                                
                                trans.builder.def_var(*var1, result);
                            }
                            OperatorType::Eq => {
                                println!("eq variables: {} {}", token1_ident.text, token3_ident.text);
                                trans.builder.def_var(*var1, val2);
                            }
                            _ => {
                                
                            }
                        }

                    }
                    else if let Token::NumberToken(token3_ident) = token3 {
                        // Declare the first variable if it is not already defined
                        if !trans.variables.contains_key(token1_ident.text.as_str()) {
                            let var = Variable::new(var_iter);
                            var_iter += 1;
                            trans.variables.insert(token1_ident.text.as_str().into(), var);
                            trans.builder.declare_var(var, I32);
                        }

                        // Declare number as const
                        let num = token3_ident.num as i64;
                        let val2 = trans.builder.ins().iconst(I32, num);

                        // Get the variable to be modified
                        let var1 = trans.variables.get(token1_ident.text.as_str()).unwrap();
                                
                        match token2_ident.op_type {
                            OperatorType::Add => {
                                println!("add variable with number: {} {}", token1_ident.text, token3_ident.num);
                                let val1 = trans.builder.use_var(*var1);
                                // Add variables and store result
                                let result = trans.builder.ins().iadd(val1, val2);
                                // Set var1 = result                                
                                trans.builder.def_var(*var1, result);
                            }
                            OperatorType::Sub => {
                                println!("sub variable with number: {} {}", token1_ident.text, token3_ident.num);
                                let val1 = trans.builder.use_var(*var1);
                                // Add variables and store result
                                let result = trans.builder.ins().isub(val1, val2);
                                // Set var1 = result                                
                                trans.builder.def_var(*var1, result);
                            }
                            OperatorType::Eq => {
                                println!("eq variable with number: {} {}", token1_ident.text, token3_ident.num);
                                trans.builder.def_var(*var1, val2);
                            }
                            _ => {
                            }
                        }
                    }
                }
            }
        }
    }

    // Get a variable to use as return value, what the function will return
    let return_value = trans.builder.use_var(var);

    // Insert the return instruction.
    trans.builder.ins().return_(&[return_value]);

    // Tell the builder we're done with this function.
    trans.builder.finalize();

    // Declare the function using the context, so that it can be called
    let name = String::from("some_other_func");
    let id = jit
        .module
        .declare_function(&name, Linkage::Export, &jit.ctx.func.signature)
        .map_err(|e| e.to_string()).unwrap();

    // Define the function, not sure what it does
    let result = jit.module
    .define_function(id, &mut jit.ctx, &mut codegen::binemit::NullTrapSink {});

    // Make sure it does not give any errors
    if result.is_err() {
        let err = result.err().unwrap();
        if let ModuleError::Compilation(CodegenError::Verifier(VerifierErrors{0: verifier_errors} )) = err {
            for i in verifier_errors.iter() {
                println!("(VerifierError");
                println!("message: {}", i.message);
                println!("context: {}", i.context.as_ref().unwrap_or(&"".to_string()));
                println!("location: {})", i.location);
            }
        }
        else {
            println!("{}", err.to_string() );
        }
        //println!("{}", result.err().unwrap().to_string() );
        panic!();
    }

    // Now that compilation is finished, we can clear out the context state.
    jit.module.clear_context(&mut  jit.ctx);

    // Finalize the functions which we just defined, which resolves any
    // outstanding relocations (patching in addresses, now that they're
    // available).
    jit.module.finalize_definitions();

    // We can now retrieve a pointer to the machine code.
    let code_ptr = jit.module.get_finalized_function(id);

    // Cast the raw pointer to a typed function pointer. This is unsafe, because
    // this is the critical point where you have to trust that the generated code
    // is safe to be called.
    unsafe {
        type I = ();
        let input: I = ();
        type O = i32;
        //let result: O = ();
        let code_fn = mem::transmute::<_, fn(I) -> O>(code_ptr);

        // And now we can call it!
        println!("Calling Program!:");
        let result = code_fn(input);
        println!("return: {}", result);
        println!(":Program ended!");
    }
}

struct FunctionTranslator<'a> {
    int: types::Type,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
}

/*pub fn print_message(word: Word) -> () {

}*/

// In order to get nested blocks, add variables as inputs into block from previous block

/*impl JIT {
    pub fn declare_call_function(&mut self) -> FuncId {
        // Declare a variable type to use for input and return
        let int = self.module.target_config().pointer_type();

        // Not sure the difference between these...
        // Create new context, that will contain all info about the function
        let mut ctx = self.module.make_context();
        ctx.func.signature.returns.push(AbiParam::new(int));
        // Create the function building context, needed to build function
        let mut builder_context = FunctionBuilderContext::new();
        // Create the builder to build the function.
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        // Create block, to start emitting code in.
        let entry_block = builder.create_block();

        // Since this is the entry block, add block parameters corresponding to
        // the function's parameters.
        builder.append_block_params_for_function_params(entry_block);

        // Tell the builder to emit code in this block.
        builder.switch_to_block(entry_block);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        builder.seal_block(entry_block);

        // Declare variable to hold variables in function
        let variables: HashMap<String, Variable> = HashMap::new();

        // Declare return variable
        let mut variables = HashMap::new();
        let var = Variable::new(0);
        let name = "some_var_name";

        if !variables.contains_key(name) {
            variables.insert(name.into(), var);
            builder.declare_var(var, int);
        }

        // Now translate the statements of the function body. (By inserting instructions into trans)
        let mut trans = FunctionTranslator {
            int,
            builder,
            variables,
            module: &mut self.module,
        };

        let mut sig = trans.module.make_signature();
        sig.params.push(AbiParam::new(int));
        sig.returns.push(AbiParam::new(int));

        let the_arg = "some_var_name".to_string();
        let arg_var = trans.variables.get(&the_arg).unwrap();
        let arg_val = trans.builder.use_var(*arg_var);

        let callee = trans
            .module
            .declare_function("print_hello_world", cranelift_module::Linkage::Import, &sig)
            .map_err(|e| e.to_string()).unwrap();

        let local_callee = trans
            .module
            .declare_func_in_func(callee, &mut trans.builder.func);

        let call = trans.builder.ins().call(local_callee, &[arg_val]);
        let call = trans.builder.ins().call(local_callee, &[arg_val]);

         // Insert the type of input and output of the function
         let mut sig = trans.module.make_signature();
         sig.params.push(AbiParam::new(int));
         sig.returns.push(AbiParam::new(int));

        /*// Declare return variable
        let var = Variable::new(0);
        let name = "return_var";
        if !trans.variables.contains_key(name) {
            trans.variables.insert(name.into(), var);
            trans.builder.declare_var(var, int);
        }*/
        let return_value = trans.builder.use_var(var);

        // Insert the return instruction.
        trans.builder.ins().return_(&[return_value]);

        // Tell the builder we're done with this function.
        trans.builder.finalize();

        // Declare the function using the context, so that it can be called
        let name = String::from("some_other_func");
        let id = self
            .module
            .declare_function(&name, Linkage::Export, &ctx.func.signature)
            .map_err(|e| e.to_string()).unwrap();

        // Define the function, not sure what it does
        let result = self.module
        .define_function(id, &mut ctx, &mut codegen::binemit::NullTrapSink {})
        .map_err(|e| e.to_string());

        // Make sure it does not give any errors
        if result.is_err() {
            println!("{}", result.err().unwrap() );
            panic!();
        }

        /*let other_func_local_callee = self
            .module
            .declare_func_in_func(id, &mut builder.func);*/

        return id;
    }
    pub fn run_code(&mut self) {
        let int = self.module.target_config().pointer_type();

        let other_func_id = self.declare_call_function();
        /*// Insert the type of input and output of the function
        let mut sig = self.module.make_signature();
        sig.params.push(AbiParam::new(int));
        sig.returns.push(AbiParam::new(int));

        // Create `ExternalName` from a string.
        let name = ExternalName::testcase("some_other_func");*/


        //let other_func_call = self.module.define_function(other_func_id, &mut self.ctx, &mut codegen::binemit::NullTrapSink {});
        //other_func_call.unwrap()
        //let func_data = cranelift::prelude::ExtFuncData{name: name, signature: sig , colocated: false};

        // Our toy language currently only supports one return value, though
        // Cranelift is designed to support more.
        self.ctx.func.signature.returns.push(AbiParam::new(int));

        // Create the builder to build a function.
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        // Create the entry block, to start emitting code in.
        let entry_block = builder.create_block();

        // Since this is the entry block, add block parameters corresponding to
        // the function's parameters.
        builder.append_block_params_for_function_params(entry_block);

        // Tell the builder to emit code in this block.
        builder.switch_to_block(entry_block);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        builder.seal_block(entry_block);

        // Declare return variable
        let mut variables = HashMap::new();
        let var = Variable::new(0);
        let name = "some_var_name";

        if !variables.contains_key(name) {
            variables.insert(name.into(), var);
            builder.declare_var(var, int);
        }

        // Now translate the statements of the function body.
        let mut trans = FunctionTranslator {
            int,
            builder,
            variables,
            module: &mut self.module,
        };

        let mut sig = trans.module.make_signature();
        sig.params.push(AbiParam::new(int));
        sig.returns.push(AbiParam::new(int));

        let the_arg = "some_var_name".to_string();
        let arg_var = trans.variables.get(&the_arg).unwrap();
        let arg_val = trans.builder.use_var(*arg_var);

        let callee = trans
            .module
            .declare_function("print_hello_world", cranelift_module::Linkage::Import, &sig)
            .map_err(|e| e.to_string()).unwrap();

        let local_callee = trans
            .module
            .declare_func_in_func(callee, &mut trans.builder.func);

        let call = trans.builder.ins().call(local_callee, &[arg_val]);

        /*let res = trans.builder.inst_results(call)[0];*/

        //let call_func = self.module

        let other_func_local_callee = trans
            .module
            .declare_func_in_func(other_func_id, &mut trans.builder.func);

        let call = trans.builder.ins().call(other_func_local_callee, &[]);

        // Set up the return variable of the function. Above, we declared a
        // variable to hold the return value. Here, we just do a use of that
        // variable.
        let the_return = "some_var_name".to_string();
        let return_variable = trans.variables.get(&the_return).unwrap();
        let return_value = trans.builder.use_var(*return_variable);

        // Emit the return instruction.
        trans.builder.ins().return_(&[return_value]);

        // Tell the builder we're done with this function.
        trans.builder.finalize();


        //Next part
        let name = String::from("some_func");
        let id = self
            .module
            .declare_function(&name, Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string()).unwrap();

        let result = self.module
        .define_function(id, &mut self.ctx, &mut codegen::binemit::NullTrapSink {});

        if result.is_err() {
            let err = result.err().unwrap();
            if let ModuleError::Compilation(CodegenError::Verifier(VerifierErrors{0: verifier_errors} )) = err {
                for i in verifier_errors.iter() {
                    println!("(VerifierError");
                    println!("message: {}", i.message);
                    println!("context: {}", i.context.as_ref().unwrap_or(&"".to_string()));
                    println!("location: {})", i.location);
                }
            }
            else {
                println!("{}", err.to_string() );
            }
            //println!("{}", result.err().unwrap().to_string() );
            panic!();
        }

        // Now that compilation is finished, we can clear out the context state.
        self.module.clear_context(&mut self.ctx);

        // Finalize the functions which we just defined, which resolves any
        // outstanding relocations (patching in addresses, now that they're
        // available).
        self.module.finalize_definitions();

        // We can now retrieve a pointer to the machine code.
        let code_ptr = self.module.get_finalized_function(id);

        // Cast the raw pointer to a typed function pointer. This is unsafe, because
        // this is the critical point where you have to trust that the generated code
        // is safe to be called.
        unsafe {
            type I = ();
            let input: I = ();
            type O = ();
            let result: O = ();
            let code_fn = mem::transmute::<_, fn(I) -> O>(code_ptr);
            // And now we can call it!
            println!("Calling Program!:");
            code_fn(input);
            println!(":Program ended!");
        }
    }
}*/