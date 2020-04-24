extern crate inkwell;

use inkwell::context::Context;
use std::{path, process};

fn compile(x: u64) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();
    let function = module.add_function("main", context.i64_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);

    // type
    let i64_type = context.i64_type();
    // value
    let x_val = context.i64_type().const_int(x, false);
    // pointer
    let alloca_x = builder.build_alloca(i64_type, "variable_x");
    // set a variable
    builder.build_store(alloca_x, x_val);
    // load a variable
    let variable_x = builder.build_load(alloca_x, "variable_x").into_int_value();

    builder.build_return(Some(&variable_x));
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
    let code = String::from("10");
    println!("code: {}", code);
    compile(10);
    run("10");
    compile(77);
    run("77");
}
