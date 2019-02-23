extern crate inkwell;

use inkwell::context::Context;
use std::{path, process};

fn compile(a1: u64, a2: u64, a3: u64, a4: u64) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    // generate function prototype
    let function = module.add_function("main", context.i64_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(&function, "entry");
    builder.position_at_end(&basic_block);

    // define main function
    let i64_type = context.i64_type();
    let a1 = i64_type.const_int(a1, false);
    let a2 = i64_type.const_int(a2, false);
    let a3 = i64_type.const_int(a3, false);
    let a4 = i64_type.const_int(a4, false);
    let sum = builder.build_int_add(a1, a2, "sum");
    let sum = builder.build_int_add(sum, a3, "sum");
    let sum = builder.build_int_add(sum, a4, "main");

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
    compile(10, 20, 30, 40);
    run("100");
}
