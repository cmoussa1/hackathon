use serde_yaml; // 0.8.7
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let f = std::fs::File::open("something.yaml")?;
    let d: String = serde_yaml::from_reader(f)?;
    println!("Read YAML string: {}", d);

    let s =
    "
    version: 1
    resources:
      - type: slot
        count: 1
        label: foo
        with:
          - type: node
            count: 1
    tasks:
      - command: [ \"app\" ]
        slot: foo
        count:
          per_slot: 1
    attributes:
    ";

    let docs = YamlLoader::load_from_str(s).unwrap();

    // Multi document support, doc is a yaml::Yaml
    let doc = &docs[0];

    // Chained key/array access is checked and won't panic,
    // return BadValue if they are not exist.
    assert!(doc["INVALID_KEY"][100].is_badvalue());

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);

    Ok(())
}
