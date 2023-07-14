use slint_build::*;

fn main() {
    let config = CompilerConfiguration::new().with_style("fluent".into());

    compile_with_config("src/ui/main.slint", config).unwrap();
}
