use yaml_rust::Yaml;
use command::{Command, Window};

#[cfg(test)] use yaml_rust::{YamlLoader};

pub fn main(yaml_string: &Vec<Yaml>) -> Vec<Command> {
    let mut commands: Vec<Command> = vec!();

    for doc in yaml_string {
        for window in doc["windows"].as_vec().unwrap() {
            match window {
                &Yaml::Hash(ref h)  => {
                    for (k, _) in h {
                        commands.push(Command::Window(Window{value: k.as_str().unwrap().to_string()}))
                    }
                },
                &Yaml::String(ref s) => {
                    commands.push(Command::Window(Window{value: s.clone()}))
                },
                &Yaml::Integer(ref s) => {
                    commands.push(Command::Window(Window{value: s.to_string()}))
                },
                _ => panic!("nope")
            };
        };

        //if doc["root"].as_str().is_some() {
        //    for c in commands.clone() {
        //        commands.push(Command::Root{value: doc["root"].as_str().unwrap().to_string(), window: c.value});
        //    };
        //};
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
