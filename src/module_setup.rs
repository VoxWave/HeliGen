struct Config {
    modules: Vec<Module>,
    output: Output,
}
struct Module {
    id: String,
    module_path: String,
    dependencies: Option<Vec<Dependency>>,
}

struct Dependency {

}

struct Output {
    module: String,
    port: usize,
}