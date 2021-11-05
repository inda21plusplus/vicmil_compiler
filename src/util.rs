use cranelift::codegen::CodegenError;
use cranelift::codegen::verifier::{VerifierErrors};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, FuncId, Linkage, Module, ModuleError};
use std::collections::HashMap;
use core::mem;
pub struct IncludedFunctions {
    func: Vec<(*const u8, String)>
}

impl IncludedFunctions {
    pub fn new() -> Self {
        Self {
            func: vec!()
        }
    }
    // Just do func_name as *const u8 to get the function
    pub fn add_function(&mut self, name: &str, func: *const u8) {
        self.func.push((func, name.to_string()));
    }
}

pub struct JIT {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    pub builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    pub ctx: codegen::Context,

    /// The data context, which is to data objects what `ctx` is to functions.
    pub _data_ctx: DataContext,

    /// The module, with the jit backend, which manages the JIT'd
    /// functions.
    pub module: JITModule,
}

pub struct CompilerUtil {
    pub jit: JIT
}


pub struct FunctionCreator<'a> {
    func_builder: &'a mut FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    variable_count: usize,
    module: &'a mut JITModule,
}

impl<'a> FunctionCreator<'a> {
    // You need to set function builder yourself,
    // Create it using FunctionBuilder::new(&mut self.jit.ctx.func, &mut self.jit.builder_context)
    pub fn new(builder: &'a mut FunctionBuilder<'a>, module: &'a mut JITModule) -> Self {
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

        Self {
            func_builder: builder,
            variables: HashMap::new(),
            variable_count: 0,
            module,
        }
    }
    pub fn finalize_func(&mut self, return_value: Value) {
        // Insert the return instruction.
        self.func_builder.ins().return_(&[return_value]);

        // Tell the builder we're done with this function.
        self.func_builder.finalize();
    }
    pub fn get_value_from_int(&mut self, num: i64) -> Value {
        use cranelift::prelude::types::I32;
        let val1 = self.func_builder.ins().iconst(I32, num);
        return val1;
    }
    pub fn get_value_from_variable(&mut self, name: &str) -> Value {
        let var = self.variables.get(name).unwrap();
        let val = self.func_builder.use_var(*var);
        return val;
    }
    pub fn add_int_values(&mut self, val1: Value, val2: Value) -> Value {
        self.func_builder.ins().iadd(val1, val2)
    }
    pub fn sub_int_values(&mut self, val1: Value, val2: Value) -> Value {
        self.func_builder.ins().isub(val1, val2)
    }
    pub fn mul_int_values(&mut self, val1: Value, val2: Value) -> Value {
        self.func_builder.ins().imul(val1, val2)
    }
    pub fn assign_value(&mut self, name: &str, value: Value) {
        let var = self.variables.get(name).unwrap();
        self.func_builder.def_var(*var, value);
    }
    pub fn create_variable(&mut self, name: &str) {
        use cranelift::prelude::types::I32;
        let var = Variable::new(self.variable_count);

        // Declare var in variables for function
        if !self.variables.contains_key(name) {
            self.variables.insert(name.into(), var);
            self.func_builder.declare_var(var, I32);
        }
    }
}

impl CompilerUtil {
    // Just input None if you have no included functions
    pub fn new(included_functions: Option<IncludedFunctions>) -> Self {
        let mut builder = JITBuilder::new(cranelift_module::default_libcall_names());

        if included_functions.is_some() {
            for i in 0..included_functions.as_ref().unwrap().func.len() {
                // Register the function
                builder.symbol(included_functions.as_ref().unwrap().func[i].1.clone(), 
                included_functions.as_ref().unwrap().func[i].0);
            }
        }
        
        // Create the JIT
        let module = JITModule::new(builder);
        let jit = JIT{
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            _data_ctx: DataContext::new(),
            module,
        };

        Self {
            jit
        }
    }

    pub fn new_function(&mut self, return_signature: Option<cranelift::codegen::ir::types::Type>) {
        use cranelift::prelude::types::I32;
        if return_signature.is_some() {
            self.jit.ctx.func.signature.returns.push(AbiParam::new(return_signature.unwrap()));
        }
        else {
            // Just use a default i32 return value
            self.jit.ctx.func.signature.returns.push(AbiParam::new(I32));
        }
    }

    pub fn end_function(&mut self, name: String) -> FuncId {
        // Declare the function using the context, so that it can be called
        let id = self
            .jit
            .module
            .declare_function(&name, Linkage::Export, &self.jit.ctx.func.signature)
            .map_err(|e| e.to_string()).unwrap();

        // Define the function, not sure what it does
        let result = self.jit.module
        .define_function(id, &mut self.jit.ctx, &mut codegen::binemit::NullTrapSink {});

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

        return id;
    }
    pub fn end_program(&mut self) {
        // Finalize the functions which we just defined, which resolves any
        // outstanding relocations (patching in addresses, now that they're
        // available).
        self.jit.module.finalize_definitions();
    }

    pub fn run_code(&mut self, id: FuncId) -> i32 {
        // Retrieve a pointer to the machine code.
        let code_ptr = self.jit.module.get_finalized_function(id);

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
            let result = code_fn(input);
            return result;
        }
    }


}