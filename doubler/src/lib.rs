#[link(wasm_import_module = "heligen")]
extern "C" { 
    fn output(channel: u64, output: f64);
    fn input(channel:u64) -> f64;
}

#[no_mangle]
fn heligen_start() {
    loop {
        let input = unsafe { input(0) };
        unsafe{
            output(0, input);
            output(0, input);
        }
    }
}
