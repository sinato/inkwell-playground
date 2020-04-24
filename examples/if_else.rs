extern crate inkwell;

use inkwell::context::Context;
use inkwell::IntPredicate;
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

    // create condition by comparing const_x and const_y
    let cond = builder.build_int_compare(IntPredicate::EQ, const_x, const_y, "ifcond");

    let then_block = context.append_basic_block(function, "then");
    let else_block = context.append_basic_block(function, "else");
    let cont_block = context.append_basic_block(function, "cont");

    builder.build_conditional_branch(cond, then_block, else_block);

    builder.position_at_end(then_block);
    builder.build_unconditional_branch(cont_block); // emit br

    builder.position_at_end(else_block);
    builder.build_unconditional_branch(cont_block); // emit br

    builder.position_at_end(cont_block);

    let then_val = i64_type.const_int(88, false);
    let else_val = i64_type.const_int(77, false);
    let phi = builder.build_phi(context.i64_type(), "iftmp");
    phi.add_incoming(&[(&then_val, then_block), (&else_val, else_block)]);

    builder.build_return(Some(&phi.as_basic_value().into_int_value()));

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
    compile(10, 10);
    run("88");
    compile(10, 20);
    run("77");
    compile(20, 10);
    run("77");
}
