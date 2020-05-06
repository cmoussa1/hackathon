extern crate yaml_rust;

use std::fs::read_to_string;
use std::fmt::Debug;

use yaml_rust::{YamlLoader};

#[derive(Debug)]
enum DependencyType {
    In,
    Out,
    InOut,
}

#[derive(Debug)]
enum DependencyScope {
    User,
    Global,
}

#[derive(Debug)]
enum DependencyScheme {
    String,
    Fluid,
}

#[derive(Debug)]
pub struct Dependency {
    dep_type: DependencyType,
    scope: DependencyScope,
    scheme: DependencyScheme,
    value: String,
}

// iterate through dependencies array in YAML file and grab hash map
// for dependencies array in YAML file:
//     for each depdendency hash map:
//         grab each key-value pair
//         push values to vector of dependencies
// return vector
pub fn parse_yaml(yaml_file: &str) -> Vec<Dependency> {
    let yaml_file = read_to_string(yaml_file).expect("Unable to read file");
    let contents = YamlLoader::load_from_str(&yaml_file).unwrap();

    let mut dependencies: Vec<Dependency> = Vec::new();

    let dependencies_in_hash_map =
        contents[0]["attributes"]["system"]["dependencies"].as_vec().unwrap();

    for hash_map in dependencies_in_hash_map {

        // match type to DependencyType enum
        let d_type = match hash_map["type"].as_str().unwrap() {
            "in" => Ok(DependencyType::In),
            "out" => Ok(DependencyType::Out),
            "inout" => Ok(DependencyType::InOut),
            _ => Err("error: the type is something else"),
        };

        // match scope to DependencyScope enum
        let d_scope = match hash_map["scope"].as_str().unwrap() {
            "user" => Ok(DependencyScope::User),
            "global" => Ok(DependencyScope::Global),
            _ => Err("error: the scope is something else"),
        };

        // match scope to DependencyScheme enum
        let d_scheme = match hash_map["scheme"].as_str().unwrap() {
            "string" => Ok(DependencyScheme::String),
            "fluid" => Ok(DependencyScheme::Fluid),
            _ => Err("error: the scheme is something else"),
        };

        // assign value as string to dependency
        let value = hash_map["value"].as_str().unwrap();

        // create Dependency object with attributes extracted
        // from HashMap
        let dependency = Dependency {
            dep_type: d_type.unwrap(),
            scope: d_scope.unwrap(),
            scheme: d_scheme.unwrap(),
            value: value.to_string(),
        };

        // push dependency to vector of dependencies
        dependencies.push(dependency);
    }

    dependencies

}
