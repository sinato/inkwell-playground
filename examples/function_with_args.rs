extern crate inkwell;

use inkwell::context::Context;
use std::{path, process};

fn compile(x: u64, y: u64) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    // type definition
    let i64_type = context.i64_type();

    // const value definition
    let const_x = i64_type.const_int(x, false); // return value
    let const_y = i64_type.const_int(y, false); // return value

    // function1
    let fn_value = module.add_function(
        "func",
        i64_type.fn_type(&[i64_type.into(), i64_type.into()], false),
        None,
    );
    let entry_bb = context.append_basic_block(fn_value, "entry_func");
    builder.position_at_end(entry_bb);
    let arg0 = fn_value.get_first_param().unwrap().into_int_value();
    let arg1 = fn_value.get_nth_param(1).unwrap().into_int_value();
    let sum = builder.build_int_add(arg0, arg1, "add");
    builder.build_return(Some(&sum));

    // main function
    let function = module.add_function("main", context.i64_type().fn_type(&[], false), None);
    let main_bb = context.append_basic_block(function, "entry");
    builder.position_at_end(main_bb);
    let func_call_site =
        builder.build_call(fn_value, &[const_x.into(), const_y.into()], "run_func");

    let val = func_call_site.try_as_basic_value().left().unwrap();

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
    compile(10, 20);
    run("30");
}
