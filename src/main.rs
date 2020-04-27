use serde_yaml; // 0.8.7
extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};
// use serde_yaml::Value;

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

    // Debug support
    println!("{:?}", doc);

    // let basic = std::fs::File::open("basic.yaml")?;
    // let x: Value = serde_yaml::from_reader(basic)?;
    // let o_docs = YamlLoader::load_from_str(x).unwrap();

    // Index access for map & array
    // assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
    // assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

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
