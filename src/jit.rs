use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
use std::collections::HashMap;
use std::{result, slice};
use core::mem;

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

        // Register hello worbuilderld print function.
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

impl JIT {
    pub fn run_code(&mut self) {
        let int = self.module.target_config().pointer_type();
        // Our toy language currently only supports one return value, though
        // Cranelift is designed to support more.
        self.ctx.func.signature.returns.push(AbiParam::new(int));

        // Create the builder to build a function.
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        // Create the entry block, to start emitting code in.
        let entry_block = builder.create_block();

        // Since this is the entry block, add block parameters corresponding to
        // the function's parameters.
        //
        // TODO: Streamline the API here.
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
        .define_function(id, &mut self.ctx, &mut codegen::binemit::NullTrapSink {})
        .map_err(|e| e.to_string());

        if result.is_err() {
            println!("{}", result.err().unwrap() );
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
}

struct FunctionTranslator<'a> {
    int: types::Type,
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
}