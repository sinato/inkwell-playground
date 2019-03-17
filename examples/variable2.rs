extern crate inkwell;

use inkwell::context::Context;
use inkwell::values::IntValue;
use std::{path, process};
use std::collections::HashMap;

fn compile(x: u64) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();
    let function = module.add_function("main", context.i64_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(&function, "entry");

    builder.position_at_end(&basic_block);

    // env
    let mut env: HashMap<&str, IntValue> = HashMap::new();

    // type
    let i64_type = context.i64_type();
    let const_x = i64_type.const_int(x, false);
    env.insert("x", const_x);
    let const_y = i64_type.const_int(100, false);
    let sum = builder.build_int_add(*env.get("x").unwrap(), const_y, "main");

    builder.build_return(Some(&sum));
    // print_to_file
    let _ = module.print_to_file(path::Path::new("compiled.ll"));
}

fn run(expect: &str) {
    // run generated IR and get returned status code
    let status = process::Command::new("sh")
        .arg("-c")
        .arg("llvm-as compiled.ll; lli compiled.bc")
        .status()
        .expect("failed to execute process");

    println!("{:?} => {:?}", status.to_string(), expect);
    assert!(status.to_string() == String::from(format!("exit code: {}", expect)));
}

fn main() {
    let code = String::from("10");
    println!("code: {}", code);
    compile(10);
    run("110");
    compile(77);
    run("177");
}
