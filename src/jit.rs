use cranelift::codegen::CodegenError;
use cranelift::codegen::verifier::{VerifierError, VerifierErrors};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, FuncId, Linkage, Module, ModuleError};
//use core::num::dec2flt::number::Number;
use std::collections::HashMap;
use std::{result, slice};
use core::mem;

pub enum Expr {
    Operation(Operation),
    IdentifierToken(IdentifierToken),
    NumberToken(NumberToken),
    Empty
}

impl Expr {
    pub fn to_string(&self) -> String {
        match &self {
            Expr::IdentifierToken(token) => {
                return token.text.clone();
            }
            Expr::NumberToken(token) => {
                return token.num.to_string();
            }
            Expr::Operation(token) => {
                let arg1 = token.expr1.to_string();
                let arg2 = token.expr2.to_string();
                let operator = "Operator".to_string();
                let mut return_string = "(".to_string();
                return_string += arg1.as_str();
                return_string += " ";
                return_string += operator.as_str();
                return_string += " ";
                return_string += arg2.as_str();
                return_string += ")";

                return return_string;
            }
            Expr::Empty => {
                return "Empty".to_string();
            }
        }
    }
}
pub struct Operation {
    expr1: Box<Expr>,
    operator: OperatorToken,
    expr2: Box<Expr>
}

#[derive(Debug, Clone)]
pub enum Token {
    IdentifierToken(IdentifierToken),
    NumberToken(NumberToken),
    OperatorToken(OperatorToken),
    EndExpr,
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::IdentifierToken(token) => {
                return "Identifier".to_string();
            }
            Token::NumberToken(token) => {
                return "Number".to_string();
            }
            Token::OperatorToken(token) => {
                return "Operator".to_string();
            }
            Token::EndExpr => {
                return "EndExpr".to_string();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierToken {
    pub text: String
}

#[derive(Debug, Clone)]
pub struct NumberToken {
    pub num: i32
}

#[derive(Debug, Clone)]
pub enum OpType0 {
    Div,
    Mul,
}

#[derive(Debug, Clone)]
pub enum OpType1 {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub enum OpType2 {
    Eq,
    AddEq,
    SubEq
}


#[derive(Debug, Clone)]
pub enum OperatorType {
    OpType0(OpType0),
    OpType1(OpType1),
    OpType2(OpType2)
}

impl OperatorType {
    pub fn type_number(&self) -> usize {
        match self {
            OperatorType::OpType0(..) => {
                return 0;
            }
            OperatorType::OpType1(..) => {
                return 1;
            }
            OperatorType::OpType2(..) => {
                return 2;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperatorToken {
    pub op_type: OperatorType
}

pub fn is_end_of_expression(my_str: String) -> Option<Token> {
    if my_str.len() == 0 {
        return None;
    }
    match my_str.as_str() {
        ";" => {
            return Some(Token::EndExpr);
        }
        _ => {
            return None;
        }
    }
    
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
        "+=" => { op_type = OperatorType::OpType2(OpType2::AddEq); }
        "-=" => { op_type = OperatorType::OpType2(OpType2::AddEq); }
        "=" => { op_type = OperatorType::OpType2(OpType2::Eq); }
        "+" => { op_type = OperatorType::OpType1(OpType1::Add); }
        "-" => { op_type = OperatorType::OpType1(OpType1::Sub); }
        "*" => { op_type = OperatorType::OpType0(OpType0::Mul); }
        "/" => { op_type = OperatorType::OpType0(OpType0::Div); }
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

pub struct TokenList {
    tokens: Vec<Token>,
    curr: usize
}

impl TokenList {
    pub fn new() -> TokenList {
        TokenList { tokens: vec!(), curr: 0 }
    }
    pub fn get_curr(&self) -> Option<Token> {
        if self.is_end() {
            return None;
        }
        return Some(self.tokens[self.curr].clone());
    }
    // Get the current token and go to next token
    pub fn get_curr_inc(&mut self) -> Option<Token> { // Get the current and icrement pointer
        if self.is_end() {
            return None;
        }
        let curr_token = Some(self.tokens[self.curr].clone());
        self.curr += 1;
        return curr_token;
    }
    pub fn inc_curr(&mut self) {
        self.curr += 1;
    }
    pub fn is_end(&self) -> bool {
        if self.curr >= self.tokens.len() {
            return true;
        }
        else {
            return false;
        }
    }
    pub fn get(&mut self, i: usize) -> Option<Token> {
        if i >= self.tokens.len() {
            return None;
        }
        else {
            return Some(self.tokens[i].clone());
        }
    }
    pub fn to_string(&self) -> String {
        let mut return_str = "{".to_string();
        for i in 0..self.tokens.len() {
            if i != 0 {
                return_str += ", ";
            }
            return_str += self.tokens[i].to_string().as_str();
        }
        return_str += "}";
        return return_str;
    }
}

pub fn recursive_generate_tree(tokens: &mut TokenList, arg1_input: Option<Expr>) -> Result<Expr, String> {
    let arg1: Expr;
    if arg1_input.is_some() {
        arg1 = arg1_input.unwrap()
    }
    else {
        match tokens.get_curr_inc() {
            Some(Token::IdentifierToken(token)) => {
                arg1 = Expr::IdentifierToken(token);
            }
            Some(Token::NumberToken(token)) => {
                arg1 = Expr::NumberToken(token);
            }
            Some(Token::EndExpr) | None => {
                tokens.curr += 1;
                if arg1_input.is_some() {
                    return Ok(arg1_input.unwrap())
                }
                return Ok(Expr::Empty);
            }
            _ => {
                return Err("invalid type".to_string());
            }
        }
    }
    let operator: OperatorToken;
    match tokens.get_curr_inc() {
        Some(Token::OperatorToken(token)) => {
            operator = token;
        }
        Some(Token::EndExpr) | None => {
            return Ok(arg1);
        }
        _ => {
            return Err("invalid operator".to_string());
        }
    }

    let arg2: Expr;
    match tokens.get_curr_inc() {
        Some(Token::IdentifierToken(token)) => {
            arg2 = Expr::IdentifierToken(token);
        }
        Some(Token::NumberToken(token)) => {
            arg2 = Expr::NumberToken(token);
        }
        Some(Token::EndExpr) | None => {
            return Err("expected identifier after operator".to_string());;
        }
        _ => {
            return Err("invalid identifier".to_string());
        }
    }

    // To see if the next operator should execute first
    match tokens.get(tokens.curr) {
        Some(Token::OperatorToken(token)) => {
            // The next operator should execute first
            if token.op_type.type_number() < operator.op_type.type_number() {
                let arg2 = recursive_generate_tree(tokens, Some(arg2));
                if arg2.is_err() {
                    return arg2;
                }
                return Ok(Expr::Operation(Operation{expr1: Box::new(arg1), expr2: Box::new(arg2.unwrap()), operator: operator}));
            }
            else if token.op_type.type_number() >= operator.op_type.type_number() {
                let this_expr = Expr::Operation(Operation{expr1: Box::new(arg1), expr2: Box::new(arg2), operator: operator});
                let result = recursive_generate_tree(tokens, Some(this_expr));
                return result;
            }
        }
        Some(Token::EndExpr) | None => {
            tokens.curr += 1;
            return Ok(Expr::Operation(Operation{expr1: Box::new(arg1), expr2: Box::new(arg2), operator: operator}));
        }
        _ => {
            return Err("invalid operator".to_string());
        }
    }
    return Ok(arg1);
}

pub fn generate_tree(tokens: &mut TokenList) -> Result<Vec<Expr>, String> {
    let mut return_vec: Vec<Expr> = vec!();
    while !tokens.is_end() {
        let expr = recursive_generate_tree(tokens, None);
        match expr {
            Err(err) => {
                return Err(err);
            }
            Ok(Expr::Empty) => {
                continue;
            }
            _ => {
                println!("{}", expr.as_ref().unwrap().to_string());
                return_vec.push(expr.unwrap());
            }
        }
    }
    return Ok(return_vec)
}

pub fn cranelift_recursive_treverse_tree(expr: &Expr, trans: &mut FunctionTranslator) -> Result<Value, String> { // Returns a value
    use cranelift::prelude::types::I32;
    match expr {
        Expr::IdentifierToken(token) => {
            // Get the identifier value and return it
            let var1 = trans.variables.get(token.text.as_str()).unwrap();
            let val1 = trans.builder.use_var(*var1);
            return Ok(val1);
        }
        Expr::NumberToken(token) => {
            // Get the number value and return it
            let num = token.num as i64;
            let val1 = trans.builder.ins().iconst(I32, num);
            return Ok(val1);

        }
        Expr::Operation(token) => {
            // Get the first value
            let val1 = cranelift_recursive_treverse_tree(&token.expr1, trans);

            // Get the second value
            let val2= cranelift_recursive_treverse_tree(&token.expr2, trans);

            // Match operator and perform operation
            match token.operator.op_type {
                OperatorType::OpType2(..) => {
                    // You cannot parse =, +=, -= inside expression
                    return Err("Cannot parse assign operation inside expression!".to_string());
                }
                OperatorType::OpType0(OpType0::Mul) => {
                    // You cannot parse =, +=, -= inside expression
                    let result = trans.builder.ins().imul(val1.unwrap(), val2.unwrap());
                    return Ok(result)
                }
                OperatorType::OpType1(OpType1::Add) => {
                    // Add values
                    let result = trans.builder.ins().iadd(val1.unwrap(), val2.unwrap());
                    return Ok(result)
                }
                OperatorType::OpType1(OpType1::Sub) => {
                    // Subtract values
                    let result = trans.builder.ins().isub(val1.unwrap(), val2.unwrap());
                    return Ok(result)
                }
                _ => {
                    return Err("Operator implemented yet!".to_string());
                }
            }
        }
        _ => {
            return Err("Could not parse expression".to_string());
        }
    }
}

pub fn cranelift_treverse_tree(expr_tree: &Vec<Expr>) -> Result<(JIT, FuncId), String> {
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

    // Declare variables variable
    let mut variables: HashMap<String, Variable> = HashMap::new();

    // Declare a variable for returning stuff
    let var = Variable::new(0);
    let name = "return_var";

    // Declare var in variables for function
    if !variables.contains_key(name) {
        variables.insert(name.into(), var);
        builder.declare_var(var, I32);
    }

    // Create a struct to keep track of variables used to create the function
    // (translate your own code of your language into cranelift)
    let mut trans: FunctionTranslator = FunctionTranslator {
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

    for expr in expr_tree {
        match expr {
            Expr::IdentifierToken(..) | Expr::NumberToken(..) | Expr::Empty => {
                continue;
            }
            Expr::Operation(op_token) => {
                match &op_token.operator.op_type {
                    OperatorType::OpType2(op_type) => {
                        // Operation is an assign operation, =, +=, -= etc.
                        match op_token.expr1.as_ref() {
                            Expr::IdentifierToken(token) => {
                                // Get the value to use
                                let val2 = cranelift_recursive_treverse_tree(&op_token.expr2, &mut trans);

                                // Get the variable to assign to
                                let name = token.text.as_str();
                                if !trans.variables.contains_key(name) {
                                    let var = Variable::new(var_iter);
                                    var_iter += 1;
                                    trans.variables.insert(name.into(), var);
                                    trans.builder.declare_var(var, I32);
                                }
                                let var1 = trans.variables.get(name).unwrap();

                                // Perform the operation
                                match op_type {
                                    OpType2::Eq => {
                                        trans.builder.def_var(*var1, val2.unwrap());
                                    }
                                    OpType2::AddEq => {
                                        let val1 = trans.builder.use_var(*var1);
                                        let result = trans.builder.ins().iadd(val1, val2.unwrap());
                                        trans.builder.def_var(*var1, result);
                                    }
                                    OpType2::SubEq => {
                                        let val1 = trans.builder.use_var(*var1);
                                        let result = trans.builder.ins().isub(val1, val2.unwrap());
                                        trans.builder.def_var(*var1, result);
                                    }
                                    _ => {
                                        println!("Not implemented yet!");
                                        panic!();
                                    }
                                }

                            }
                            _ => {
                                println!("Expected identifier!");
                                panic!();
                            }
                        }
                    }
                    _ => {
                        // We dont care about the output, since the expression is for example x + y, and
                        // it does not assign to anything
                        cranelift_recursive_treverse_tree(&expr, &mut trans);
                    }
                }
            }
            _ => {
                continue;
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

    return Ok((jit, id));
}

pub fn tokenize(my_str: String) -> TokenList {
    let mut token_list = TokenList::new();
    let split_str = my_str.split_ascii_whitespace();
    for i in split_str.into_iter() {
        if let Some(token) = is_operator(i.to_string()) {
            println!("operator: {}", i);
            token_list.tokens.push(Token::OperatorToken(token));
        }
        else if let Some(token) = is_number(i.to_string()) {
            println!("number: {}", i);
            token_list.tokens.push(Token::NumberToken(token));
        }
        else if let Some(token) = is_identifier(i.to_string()) {
            println!("identifier: {}", i);
            token_list.tokens.push(Token::IdentifierToken(token));
        }
        else if let Some(token) = is_end_of_expression(i.to_string()) {
            println!("EndOfExpression: {}", i);
            token_list.tokens.push(token);
        }
        else {
            println!("undefined: {}", i);
        }
    }
    return token_list;
}

pub fn run_code(id: FuncId, jit: JIT) {
    // Retrieve a pointer to the machine code.
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

pub fn compile_code(my_str: String) {
    // Use the lexer to split up string into tokens
    let mut token_list = tokenize(my_str);
    println!("tokens: {}", token_list.to_string());

    // Generate expression tree
    let tree = generate_tree(&mut token_list);

    // Treverse tree with cranelift to generate executable function
    let (jit, id) = cranelift_treverse_tree(&tree.unwrap()).unwrap();

    // Run the function
    run_code(id, jit);
}

pub struct FunctionTranslator<'a> {
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

        /* // Declare return variable
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
        /* // Insert the type of input and output of the function
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

/*let mut var_iter = 1;
    // Parse the code from the custom language
    if token_list.tokens.len() >= 3 {
        for i in 0..token_list.tokens.len()-2 {
            let token1: &Token = token_list.tokens.get(i).unwrap();
            let token2: &Token = token_list.tokens.get(i+1).unwrap();
            let token3: &Token = token_list.tokens.get(i+2).unwrap();
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
                        match &token2_ident.op_type {
                            OperatorType::OpType2(op_type) => {
                                match op_type {
                                    OpType2::AddEq => {
                                        println!("add variables: {} {}", token1_ident.text, token3_ident.text);
                                        let val1 = trans.builder.use_var(*var1);
                                        // Add variables and store result
                                        let result = trans.builder.ins().iadd(val1, val2);
                                        // Set var1 = result                                
                                        trans.builder.def_var(*var1, result);
                                    }
                                    OpType2::SubEq => {
                                        println!("sub variables: {} {}", token1_ident.text, token3_ident.text);
                                        let val1 = trans.builder.use_var(*var1);
                                        // Add variables and store result
                                        let result = trans.builder.ins().isub(val1, val2);
                                        // Set var1 = result                                
                                        trans.builder.def_var(*var1, result);
                                    }
                                    OpType2::Eq => {
                                        println!("eq variables: {} {}", token1_ident.text, token3_ident.text);
                                        trans.builder.def_var(*var1, val2);
                                    }
                                }
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
                            OperatorType::OpType2(OpType2::AddEq) => {
                                println!("add variable with number: {} {}", token1_ident.text, token3_ident.num);
                                let val1 = trans.builder.use_var(*var1);
                                // Add variables and store result
                                let result = trans.builder.ins().iadd(val1, val2);
                                // Set var1 = result                                
                                trans.builder.def_var(*var1, result);
                            }
                            OperatorType::OpType2(OpType2::SubEq) => {
                                println!("sub variable with number: {} {}", token1_ident.text, token3_ident.num);
                                let val1 = trans.builder.use_var(*var1);
                                // Add variables and store result
                                let result = trans.builder.ins().isub(val1, val2);
                                // Set var1 = result                                
                                trans.builder.def_var(*var1, result);
                            }
                            OperatorType::OpType2(OpType2::Eq) => {
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
    }*/