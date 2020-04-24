extern crate inkwell;

use inkwell::context::Context;
use inkwell::AddressSpace;
use std::{path, process};

fn compile(x: u64) {
    // initialize
    let context = Context::create();
    let builder = context.create_builder();

    let context = Context::create();
    let module = context.create_module("mod");
    let i64_type = context.i64_type();
    let global = module.add_global(i64_type, Some(AddressSpace::Const), "my_global");

    let x_val = context.i64_type().const_int(x, false);
    global.set_initializer(&x_val);

    let function = module.add_function("main", i64_type.fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);
    let val = builder
        .build_load(global.as_pointer_value(), "val")
        .into_int_value();

    builder.build_return(Some(&val));
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
