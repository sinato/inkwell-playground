extern crate inkwell;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::PointerValue;
use std::{path, process};

fn get_element_pointer(
    builder: &Builder,
    context: &Context,
    p: PointerValue,
    num: u64,
) -> PointerValue {
    unsafe {
        builder.build_gep(
            p,
            &[
                context.i32_type().const_int(0, false),
                context.i32_type().const_int(num, false),
            ],
            "insert",
        )
    }
}

fn compile(x: u64) {
    // initialize
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();
    let function = module.add_function("main", context.i32_type().fn_type(&[], false), None);
    let basic_block = context.append_basic_block(&function, "entry");

    builder.position_at_end(&basic_block);

    let array_type = context.i32_type().array_type(2).array_type(2);
    let parent_array_alloca = builder.build_alloca(array_type, "array_alloca");

    let p0 = get_element_pointer(&builder, &context, parent_array_alloca, 0);
    let p00 = get_element_pointer(&builder, &context, p0, 0);
    builder.build_store(p00, context.i32_type().const_int(x, false));
    let p01 = get_element_pointer(&builder, &context, p0, 1);
    builder.build_store(p01, context.i32_type().const_int(x + 1, false));

    let p1 = get_element_pointer(&builder, &context, parent_array_alloca, 1);
    let p10 = get_element_pointer(&builder, &context, p1, 0);
    builder.build_store(p10, context.i32_type().const_int(x + 2, false));
    let p11 = get_element_pointer(&builder, &context, p1, 1);
    builder.build_store(p11, context.i32_type().const_int(x + 3, false));

    let v00 = builder.build_load(p00, "v00").into_int_value();
    let v01 = builder.build_load(p01, "v01").into_int_value();
    let v10 = builder.build_load(p10, "v10").into_int_value();
    let v11 = builder.build_load(p11, "v11").into_int_value();

    let sum0 = builder.build_int_add(v00, v01, "sum0");
    let sum1 = builder.build_int_add(v10, v11, "sum1");
    let sum = builder.build_int_add(sum0, sum1, "sum");
    builder.build_return(Some(&sum));

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
    compile(10);
    run("46");
}
