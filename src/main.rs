extern crate yaml_rust;

use std::env;

mod parse_yaml;

fn main() {

    let args: Vec<String> = env::args().collect();

    // pass filename as first argument on command line
    let filename: &str = &args[1];

    // create vector of dependencies
    let dependencies: Vec<parse_yaml::Dependency> = parse_yaml::parse_yaml(filename);
    println!("{:#?}", dependencies);

}
