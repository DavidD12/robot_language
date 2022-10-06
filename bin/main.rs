use rl::*;

fn check(file: &str) {
    println!(
        "-------------------------[{}]-------------------------",
        file
    );
    let mut model = Model::empty();
    match parse_model_file(&mut model, file) {
        Ok(_) => println!("{}", model),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    check("examples/type_1.rl");
    check("examples/type_2.rl");
    check("examples/type_3.rl");
    check("examples/type_4.rl");

    check("examples/skillset_1.rl");

    check("examples/data_1.rl");
    check("examples/data_2.rl");
    check("examples/data_3.rl");

    check("examples/resource_1.rl");
}
