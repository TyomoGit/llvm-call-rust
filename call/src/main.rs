use inkwell::{context::Context, module::{Linkage, Module}, passes::PassManagerSubType, values::AsValueRef, AddressSpace, OptimizationLevel};

#[link(name = "rust_lib", kind = "static")]
extern "C" {
    fn pow(x: f64, y: f64) -> f64;
    fn mypow(x: f64, y: f64) -> f64;
}

fn main() {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();

    

    let fn_ty = context.f64_type().fn_type(&[context.f64_type().into(), context.f64_type().into()], false);
    let pow_fn =module.add_function("mypow", fn_ty, Some(Linkage::External));

    let main_fn_ty = context.i8_type().fn_type(&[], false);
    let main_fn = module.add_function("main", main_fn_ty, Some(Linkage::External));

    let printf_ty = context.i8_type().fn_type(&[context.i8_type().ptr_type(AddressSpace::default()).into()], true);
    let printf_fn = module.add_function("printf", printf_ty, None);

    let basic_block = context.append_basic_block(main_fn, "entry");
    builder.position_at_end(basic_block);

    let string = builder.build_global_string_ptr("output: %f\n", "string").unwrap();

    let result = builder.build_call(
        pow_fn,
        &[context.f64_type().const_float(2.0).into(), context.f64_type().const_float(3.0).into()],
        "call",
    ).unwrap();

    builder.build_call(printf_fn, &[string.as_pointer_value().into(), result.try_as_basic_value().left().unwrap().into()], "call").unwrap();

    builder.build_return(Some(&context.i8_type().const_zero())).unwrap();

    module.print_to_file("output.ll").unwrap();

    let engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    engine.add_global_mapping(&pow_fn, mypow as usize);
    let _output = unsafe {
        engine
            .get_function::<unsafe extern "C" fn() -> i8>("main")
            .unwrap()
            .call()
    };

    let direct_result = unsafe {
        pow(2.0, 3.0)
    };

    println!("direct: {}", direct_result);

    let direct_result = unsafe {
        mypow(2.0, 3.0)
    };

    println!("direct: {}", direct_result);
}
