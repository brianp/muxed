use yaml_rust::Yaml;
use command::{Command, Session, SendKeys, Split, Layout, Window, Window2, Attach, Panes};

#[cfg(test)] use yaml_rust::{YamlLoader};

pub fn main(yaml_string: &Vec<Yaml>, project_name: String) -> Vec<Command> {
    let mut commands: Vec<Command> = vec!();

    for doc in yaml_string {
        let root = match doc["root"].as_str() {
            Some(x) => Some(x.to_string()),
            None    => None
        };

        let (first_window, windows) = doc["windows"].as_vec().expect("No Windows have been defined.").split_at(1);

        match &first_window[0] {
             &Yaml::Hash(ref h)  => {
                 for (k, v) in h {
                     if v.as_hash().is_some() {
                         commands.push(Command::Session(Session{name: project_name.clone(), window_name: k.as_str().unwrap().to_string(), root: root.clone()}));
                         commands.append(&mut pane_matcher(project_name.clone(), v, root.clone(), k.as_str().unwrap().to_string()));
                     } else {
                         commands.push(Command::Session(Session{name: project_name.clone(), window_name: k.as_str().unwrap().to_string(), root: root.clone()}));
                         commands.push(Command::SendKeys(SendKeys{target: format!("{}:{}", project_name, k.as_str().unwrap().to_string()).to_string(), exec: v.as_str().expect("Bad exec command").to_string()}));
                     }
                 }
             },
             &Yaml::String(ref s) => {
                 commands.push(Command::Session(Session{name: project_name.clone(), window_name: s.clone(), root: root.clone()}))
             },
             &Yaml::Integer(ref s) => {
                 commands.push(Command::Session(Session{name: project_name.clone(), window_name: s.to_string(), root: root.clone()}))
             },
             _ => panic!("Muxed config file formatting isn't recognized.")
        };

        for (i,window) in windows.iter().enumerate() {
            match window {
                &Yaml::Hash(ref h)  => {
                    for (k, v) in h {
                        if v.as_hash().is_some() {
                            commands.push(Command::Window(Window{value: k.as_str().unwrap().to_string(), root: root.clone(), exec: "".to_string()}));
                            commands.append(&mut pane_matcher(project_name.clone(), v, root.clone(), k.as_str().unwrap().to_string()));
                        } else {
                            commands.push(Command::Window2(Window2{session_name: project_name.clone(), name: k.as_str().unwrap().to_string(), root: root.clone()}));
                            commands.push(Command::SendKeys(SendKeys{target: format!("{}:{}", project_name, k.as_str().unwrap().to_string()).to_string(), exec: v.as_str().expect("Bad exec command").to_string()}));
                        }
                    }
                },
                &Yaml::String(ref s) => {
                    commands.push(Command::Window2(Window2{session_name: project_name.clone(), name: s.clone(), root: root.clone()}))
                },
                &Yaml::Integer(ref s) => {
                    commands.push(Command::Window2(Window2{session_name: project_name.clone(), name: s.to_string(), root: root.clone()}))
                },
                _ => panic!("Muxed config file formatting isn't recognized.")
            };
        };
    };
    commands.push(Command::Attach(Attach{name: project_name.clone()}));
    commands
}

fn pane_matcher(session: String, panes: &Yaml, root: Option<String>, window: String) -> Vec<Command> {
    let mut commands = vec!();
    let panes2 = panes["panes"].as_vec().expect("Something is wrong with panes.");

    for (i, pane) in panes2.iter().enumerate() {
        commands.push(Command::SendKeys(SendKeys{target: format!("{}:{}.{}", session, window, i).to_string(), exec: pane.as_str().expect("Bad exec command").to_string()}));
    };

    if panes["layout"].as_str().is_some() {
        let layout = panes["layout"].as_str().unwrap().to_string();
        commands.push(Command::Layout(Layout{target: format!("{}:{}", session, window).to_string(), layout: panes["layout"].as_str().expect("Bad layout").to_string()}));
    };

    commands
}

#[test]
pub fn windows_as_array() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml, "muxed".to_string());
    assert_eq!(commands.len(), 3)
}

#[test]
pub fn windows_with_integer_names() {
    let s = "---
windows: [1, 'vim', 3]
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml, "muxed".to_string());
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
    let commands = main(&yaml, "muxed".to_string());
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
    let commands = main(&yaml, "muxed".to_string());

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
    let commands = main(&yaml, "muxed".to_string());
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
    let commands = main(&yaml, "muxed".to_string());

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
    let commands = main(&yaml, "muxed".to_string());

    let pane_command: Option<Panes> = match commands[1].clone() {
        Command::Panes(w) => Some(w),
        _                 => None
    };

    assert_eq!(pane_command.clone().unwrap().exec[0], "vim");
    assert_eq!(pane_command.clone().unwrap().exec[1], "guard")
}
