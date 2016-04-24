use std::collections::BTreeMap;
use yaml_rust::Yaml;

#[cfg(test)] use yaml_rust::{YamlLoader};

#[derive(Debug)]
pub struct Command {
    //commands: Vec<Command>,
    pub key: String,
    pub value: String
}

pub fn main(yaml_string: &Vec<Yaml>) -> Vec<Command> {
    let mut commands: Vec<Command> = vec!();

    fn yaml_match(y: &Yaml) {
        match y {
            &Yaml::Array(ref v) => {
                for x in v {
                    yaml_match(x);
                }
            },
            &Yaml::Hash(ref h) => {
                for (k, v) in h {
                    println!("hash: {:?}:", k);
                    yaml_match(v);
                }
            },
            _ => print!("{:?}", y)
        };
    };

    for doc in yaml_string {
        for window in doc["windows"].as_vec().unwrap() {
            match window {
                &Yaml::Hash(ref h)  => {
                    for (k, v) in h {
                        commands.push(Command{key: "window".to_string(), value: k.as_str().unwrap().to_string()})
                    }
                },
                &Yaml::String(ref s) => {
                    commands.push(Command{key: "window".to_string(), value: s.as_str().to_string()})
                },
                &Yaml::Integer(ref s) => {
                    commands.push(Command{key: "window".to_string(), value: s.to_string()})
                },
                _ => panic!("nope")
            };
        };
    };

    println!("{:?}", commands);
    commands
}

#[test]
pub fn windows_as_array() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml);
    assert_eq!(commands.len(), 3)
}

#[test]
pub fn windows_with_integer_names() {
    let s = "---
windows: [1, 'vim', 3]
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml);
    assert_eq!(commands.len(), 3)
}

#[test]
pub fn windows_as_list() {
    let s = "---
windows:
  - cargo: ''
  - vim: ''
  - git: ''
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml);
    assert_eq!(commands.len(), 3)
}
