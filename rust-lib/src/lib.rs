#[no_mangle]
extern "C" fn mypow(x: f64, y: f64) -> f64 {
    dbg!(x, y);
    x.powf(y)
}

// #[no_mangle]
extern "C" fn pow(x: f64, y: f64) -> f64 {
    dbg!(x, y);
    x.powf(y)
}
