use rl::*;
use std::env;

fn check(file: &str) {
    println!(
        "-------------------------[{}]-------------------------",
        file
    );
    let mut model = Model::empty();
    match process_file(&mut model, file) {
        Ok(_) => println!("{}", model),
        _ => {}
    }
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    // pretty_env_logger::init();
    env_logger::init();

    check("examples/err_type_1.rl"); // parse error
    check("examples/err_type_2.rl"); // duplicate error

    check("examples/err_data_1.rl"); // duplicate error
    check("examples/err_data_2.rl"); // resolve error

    check("examples/err_resource_1.rl"); // duplicate error (resource)
    check("examples/err_resource_2.rl"); // duplicate error (state)
    check("examples/err_resource_3.rl"); // resolve initial error
    check("examples/err_resource_4.rl"); // resolve transition src error
    check("examples/err_resource_5.rl"); // resolve transition dst error

    check("examples/err_event_1.rl"); // duplicate error (event)
}
