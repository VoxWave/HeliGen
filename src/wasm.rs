use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use wasmtime::Func;
use wasmtime::Linker;
use wasmtime::Module;
use wasmtime::Store;

pub fn spawn_wasm_thread(module_name: String, inputs: Vec<Receiver<f64>>, outputs: Vec<Sender<f64>>) {
    std::thread::spawn(move || {
        let store = Store::default();
        let mut linker = Linker::new(&store);
        linker
            .define(
                "heligen",
                "output",
                Func::wrap(&store, move |channel: u64, output: f64| {
                    match outputs.get(channel as usize) {
                        Some(channel) => channel.send(output).unwrap(),
                        None => println!(
                            "value {} sent to unconnected channel number {}",
                            output, channel
                        ),
                    };
                }),
            )
            .unwrap();
        linker
            .define(
                "heligen",
                "input",
                Func::wrap(&store, move |channel: u64| -> f64 {
                    match inputs.get(channel as usize) {
                        Some(channel) => channel.recv().unwrap(), // Figure out how to shut down cleanly when a channel closes.
                        None => 0.,
                    }
                }),
            )
            .unwrap();
        let module_file = std::fs::read(module_name).unwrap();
        let module = Module::new(store.engine(), module_file).unwrap();
        let instance = linker.instantiate(&module).unwrap();
        let start_func = instance
            .get_func("heligen_start")
            .unwrap()
            .get0::<()>()
            .unwrap();
        start_func().unwrap();
    });
}