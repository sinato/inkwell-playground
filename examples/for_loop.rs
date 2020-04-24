extern crate inkwell;

use inkwell::context::Context;
use inkwell::IntPredicate;
use std::{path, process};

fn compile(x: u64, y: u64, z: u64) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    // generate function prototype
    let function = module.add_function("main", context.i64_type().fn_type(&[], false), None);

    let basic_block = context.append_basic_block(function, "entry");
    let loop_block = context.append_basic_block(function, "loop");
    let cont_block = context.append_basic_block(function, "cont");

    let i64_type = context.i64_type();

    let x_val = context.i64_type().const_int(x, false);
    let val_one = context.i64_type().const_int(y, false);
    let val_target = context.i64_type().const_int(z, false);

    builder.position_at_end(basic_block);
    let alloca_x = builder.build_alloca(i64_type, "variable_x");
    builder.build_store(alloca_x, x_val);
    builder.build_unconditional_branch(loop_block); // emit br

    builder.position_at_end(loop_block);
    let variable_x = builder.build_load(alloca_x, "variable_x").into_int_value();
    let new_variable_x = builder.build_int_add(variable_x, val_one, "increment");
    builder.build_store(alloca_x, new_variable_x);
    let cond =
        builder.build_int_compare(IntPredicate::UGE, new_variable_x, val_target, "while_cond");
    builder.build_conditional_branch(cond, cont_block, loop_block); // emit br

    builder.position_at_end(cont_block);
    let variable_x = builder.build_load(alloca_x, "variable_x").into_int_value();
    builder.build_return(Some(&variable_x));
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
    compile(0, 10, 40);
    run("40");
    compile(3, 5, 20);
    run("23");
}
