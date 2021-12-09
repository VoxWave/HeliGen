pub mod moofloom {
    #[link(wasm_import_module = "heligen")]
    extern "C" { 
        pub fn output(channel: u64, output: f64);
    }
}

fn output(channel: u64, output: f64) {
    unsafe { moofloom::output(channel, output) }
}

#[no_mangle]
fn heligen_start() {
    loop{
        output(0,1.);
        output(0,1.);
        output(0,-1.);
        output(0,-1.)
    }
}
