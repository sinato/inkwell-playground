extern crate inkwell;

use inkwell::context::Context;
use inkwell::values::InstructionOpcode;
use std::{path, process};

fn compile(code: String) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    // generate function prototype
    let function = module.add_function("main", context.i32_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    // define main function
    let num = code.parse::<f64>().unwrap();
    let a = context.f32_type().const_float(num);

    // prepare
    let alloca = builder.build_alloca(context.f32_type(), "");
    builder.build_store(alloca, a);
    let val = builder.build_load(alloca, "");

    // cast
    let opcode = InstructionOpcode::FPToSI;
    let ret = builder.build_cast(opcode, val, context.i32_type(), "");
    builder.build_return(Some(&ret));

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
    let code = String::from("7.77");
    println!("code: {}", code);
    compile(code);
    run("7");
}
