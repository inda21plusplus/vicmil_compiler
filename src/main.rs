pub mod jit;
pub use jit::*;

fn main() {
    compile_code("x = 7 ; x += 8 + 10 * 5 + 3 ; return_var = x ; x = 3".to_string());
    println!("Program finished!");
}