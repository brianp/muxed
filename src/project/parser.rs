use std::collections::BTreeMap;
use yaml_rust::Yaml;
use tmux;

pub struct Command {
    //commands: Vec<Command>,
    key: String,
    value: String
}

pub fn main(yaml_string: &Vec<Yaml>) -> Vec<Command> {
    let mut commands: Vec<Command> = vec!();
    //println!("{:?}", yaml_string);

    //fn yaml_match(y: &Yaml) {
    //    match y {
    //        &Yaml::Array(ref v) => {
    //            for x in v {
    //                yaml_match(x);
    //            }
    //        },
    //        &Yaml::Hash(ref h) => {
    //            for (k, v) in h {
    //                println!("hash: {:?}:", k);
    //                yaml_match(v);
    //            }
    //        },
    //        _ => print!("{:?}", y)
    //    }
    //};

    //for line in yaml_string {
    //    for window in yaml_match(line["windows"]) {
    //        let value = yaml_match(window);
    //        let command = Command{key: "window", value: value};
    //        commands.push(command);
    //    };
    //};

    commands
}
