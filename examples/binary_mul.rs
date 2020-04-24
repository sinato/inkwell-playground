extern crate inkwell;

use inkwell::context::Context;
use std::{path, process};

fn compile(x: u64, y: u64) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    // generate function prototype
    let function = module.add_function("main", context.i64_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    // define main function
    let i64_type = context.i64_type();
    let const_x = i64_type.const_int(x, false);
    let const_y = i64_type.const_int(y, false);
    let sum = builder.build_int_mul(const_x, const_y, "main");
    builder.build_return(Some(&sum));

    // print_to_file
    let _ = module.print_to_file(path::Path::new("compiled.ll"));
}

fn run(expect: &str) {
    // run generated IR and get returned status code
    let status = process::Command::new("sh")
        .arg("-c")
        .arg("llvm-as-10 compiled.ll; lli-10 compiled.bc")
        .status()
        .expect("failed to execute process");

    println!("{:?} => {:?}", status.to_string(), expect);
    assert!(status.to_string() == String::from(format!("exit code: {}", expect)));
}

fn main() {
    compile(2, 7);
    run("14");
}
