extern crate inkwell;

use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::AddressSpace;
use std::{path, process};

fn compile() {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    let fn_type = context.i32_type().fn_type(
        &[context.i8_type().ptr_type(AddressSpace::Generic).into()],
        true,
    );
    module.add_function("printf", fn_type, Some(Linkage::External));

    let function = module.add_function("main", context.i32_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    let hello_str = unsafe { builder.build_global_string("hello world\n", "hello_str") };
    let fn_value = match module.get_function("printf") {
        Some(value) => value,
        None => panic!(),
    };
    let const_zero = context.i32_type().const_int(0, false);
    let _ = unsafe {
        builder.build_call(
            fn_value,
            &[hello_str
                .as_pointer_value()
                .const_gep(&[const_zero, const_zero])
                .into()],
            "run_func",
        )
    };
    builder.build_return(Some(&const_zero));
    // print_to_file
    let _ = module.print_to_file(path::Path::new("compiled.ll"));
}

fn run(expect: &str) {
    // run generated IR and get returned status code
    let output = process::Command::new("sh")
        .arg("-c")
        .arg("llvm-as-10 compiled.ll; lli-10 compiled.bc")
        .output()
        .expect("failed to execute process");

    let stdout_string = std::str::from_utf8(&output.stdout).unwrap();
    println!("{} => {}", stdout_string, expect);
    assert!(stdout_string == expect);
}

fn main() {
    compile();
    run("hello world\n");
}
