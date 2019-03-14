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

    let basic_block = context.append_basic_block(&function, "entry");
    builder.position_at_end(&basic_block);

    // define main function
    let i64_type = context.i64_type();
    let const_x = i64_type.const_int(x, false);
    let const_y = i64_type.const_int(y, false);

    // create condition by comparing const_x and const_y
    let cond = builder.build_int_compare(IntPredicate::EQ, const_x, const_y, "ifcond");

    let then_block = context.append_basic_block(&function, "then");
    let else_block = context.append_basic_block(&function, "else");
    let cont_block = context.append_basic_block(&function, "cont");

    builder.build_conditional_branch(cond, &then_block, &else_block);

    builder.position_at_end(&then_block);
    builder.build_unconditional_branch(&cont_block); // emit br

    builder.position_at_end(&else_block);
    builder.build_unconditional_branch(&cont_block); // emit br

    builder.position_at_end(&cont_block);

    let phi = builder.build_phi(context.i64_type(), "iftmp");
    let then_val = i64_type.const_int(88, false);
    let else_val = i64_type.const_int(77, false);
    phi.add_incoming(&[(&then_val, &then_block), (&else_val, &else_block)]);

    let phi_value = phi.as_basic_value().into_int_value();
    // memoary allocation
    let alloca_phi = builder.build_alloca(i64_type, "variable_phi");
    // set a variable
    builder.build_store(alloca_phi, phi_value);
    // load a variable
    let variable_x = builder.build_load(alloca_phi, "variable_phi").into_int_value();
    builder.build_return(Some(&variable_x));

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
    compile(10, 10);
    run("88");
}
