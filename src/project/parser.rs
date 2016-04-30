use yaml_rust::Yaml;
use command::{Command, Window, Panes};

#[cfg(test)] use yaml_rust::{YamlLoader};

pub fn main(yaml_string: &Vec<Yaml>) -> Vec<Command> {
    let mut commands: Vec<Command> = vec!();

    for doc in yaml_string {
        for window in doc["windows"].as_vec().unwrap() {
            let root = match doc["root"].as_str() {
                Some(x) => Some(x.to_string()),
                None    => None
            };

            commands.append(&mut window_matcher(window, &root));
        };
    };

    commands
}

fn window_matcher(window: &Yaml, root: &Option<String>) -> Vec<Command> {
    let mut commands: Vec<Command> = vec!();

    match window {
        &Yaml::Hash(ref h)  => {
            for (k, v) in h {
                if v.as_hash().is_some() {
                    commands.push(Command::Window(Window{value: k.as_str().unwrap().to_string(), root: root.clone(), exec: "".to_string()}));
                    commands.push(pane_matcher(v, root.clone(), k.as_str().unwrap().to_string()));
                } else {
                    commands.push(Command::Window(Window{value: k.as_str().unwrap().to_string(), root: root.clone(), exec: v.as_str().unwrap().to_string().clone()}));
                }
            }
        },
        &Yaml::String(ref s) => {
            commands.push(Command::Window(Window{value: s.clone(), root: root.clone(), exec: "".to_string()}))
        },
        &Yaml::Integer(ref s) => {
            commands.push(Command::Window(Window{value: s.to_string(), root: root.clone(), exec: "".to_string()}))
        },
        _ => panic!("nope")
    };

    commands
}

fn pane_matcher(panes: &Yaml, root: Option<String>, window: String) -> Command {
    let layout = panes["layout"].as_str().unwrap().to_string();
    let exec: Vec<String> = panes["panes"].as_vec().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
    Command::Panes(Panes{window: window, layout: layout, root: root, exec: exec})
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

#[test]
pub fn root_command() {
    let s = "---
root: '~/.muxed'
windows:
  - cargo: ''
  - vim: ''
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml);

    let first_window: Option<Window> = match commands[0].clone() {
        Command::Window(w) => Some(w),
        _                  => None
    };

    assert_eq!(first_window.unwrap().root.unwrap(), "~/.muxed".to_string())
}

#[test]
pub fn panes_array() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml);
    assert_eq!(commands.len(), 2)
}

#[test]
pub fn panes_command_exists() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml);

    let pane_command: Option<Panes> = match commands[1].clone() {
        Command::Panes(w) => Some(w),
        _                 => None
    };

    assert!(pane_command.is_some())
}

#[test]
pub fn panes_command_execs_array() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml);

    let pane_command: Option<Panes> = match commands[1].clone() {
        Command::Panes(w) => Some(w),
        _                 => None
    };

    assert_eq!(pane_command.clone().unwrap().exec[0], "vim");
    assert_eq!(pane_command.clone().unwrap().exec[1], "guard")
}
