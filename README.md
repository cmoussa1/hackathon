## Hackathon Project Spring 2020 - Rusting Flux pt.2

This project implements a YAML file parser for Flux jobspecs. It extracts job dependencies from the file and creates a vector of Dependency objects, which gets returned to `main()`.

### Usage

```
cargo run <file name>

[
Dependency {
   dep_type: In,
   scope: User,
   scheme: Fluid,
   value: "hungry-hippo-white-elephant",
},
Dependency {
   dep_type: In,
   scope: User,
   scheme: String,
   value: "foo",
},
Dependency {
   dep_type: Out,
   scope: Global,
   scheme: String,
   value: "bar",
},
]
```

### Background

As defined in [Flux's Job Dependency Specification](https://github.com/flux-framework/rfc/blob/master/spec_26.rst), a dependency is a dictionary containing the following keys:

- type
- scope,
- scheme,
- value

This projects performs a `match` on each key-value pair in the dependency dictionary to make sure each value is valid using **Dependency\<Key> Enums**.

### Future Work

##### YAML File Dependency Evaluation

Right now, the program assumes that the YAML file passed in will always contain a a `dependencies` key. If there is no `dependencies` key, the program panics and reports the following messy error:

`thread 'main' panicked at 'called 'Option::unwrap()' on a 'None' value'`

One idea on how to gracefully handle this error is to check for a `dependencies` key beforehand; if there is none, exit gracefully and report there is no key. Otherwise, begin evaluating and extracting dependency values.

##### Dependency HashMap Value Evaluation

The error handling in this program is not very extensive. The error message that prints out on a mismatch for Dependency values is not pretty, and should probably gracefully exit with a message saying the value evaluated does not match any of the specified values.
