extern crate inkwell;

use inkwell::context::Context;
use std::{path, process};

fn compile(x: u64) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();
    let function = module.add_function("main", context.i32_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(&function, "entry");

    builder.position_at_end(&basic_block);

    let array_type = context.i32_type().array_type(1);
    let array_alloca = builder.build_alloca(array_type, "array_alloca");

    let ptr = unsafe {
        builder.build_gep(
            array_alloca,
            &[
                context.i32_type().const_int(0, false),
                context.i32_type().const_int(0, false),
            ],
            "insert",
        )
    };
    let x_val = context.i32_type().const_int(x + 10, false);
    builder.build_store(ptr, x_val);

    let array = builder
        .build_load(array_alloca, "array_load")
        .into_array_value();
    let val0 = builder
        .build_extract_value(array, 0, "extract")
        .unwrap()
        .into_int_value();
    builder.build_return(Some(&val0));
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
    compile(7);
    run("17");
    compile(77);
    run("87");
}
