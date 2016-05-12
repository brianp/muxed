//! The YAML parser. Here is where we convert the yaml in to commands to be
/// processed later.

use yaml_rust::Yaml;
use command::{Command, Session, SendKeys, Split, Layout, Window, Attach, KillWindow};
use rand::random;

#[cfg(test)] use yaml_rust::{YamlLoader};

/// Here was pass in the parsed yaml and project name. The purpose of this main
/// loop is to build the stack of commands that are run to setup a users tmux
/// session.
///
/// yaml_string: The parsed yaml from the config file.
/// project_name: The name of the project.
pub fn main(yaml_string: &Vec<Yaml>, project_name: &String) -> Vec<Command> {
    let mut commands: Vec<Command> = vec!();
    let tmp_window_name = format!("muxed_first_window_{}", random::<u16>().to_string());

    // The initial session command. Contains the tmp_window_name to be closed
    // before attaching.
    commands.push(Command::Session(Session{name: project_name.clone(), tmp_window_name: tmp_window_name.clone()}));

    // There should only be one doc but it's a vec so loop it.
    for doc in yaml_string {
        let root = match doc["root"].as_str() {
            Some(x) => Some(x.to_string()),
            None    => None
        };

        let windows = doc["windows"].as_vec().expect("No Windows have been defined.");
        for (i, window) in windows.iter().enumerate() {
            match window {
                &Yaml::Hash(ref h)  => {
                    for (k, v) in h {
                        if v.as_hash().is_some() {
                            commands.push(Command::Window(Window{session_name: format!("{}:{}", project_name.clone(), i+1), name: k.as_str().unwrap().to_string(), root: root.clone()}));
                            commands.append(&mut pane_matcher(project_name.clone(), v, root.clone(), k.as_str().unwrap().to_string()));
                        } else {
                            commands.push(Command::Window(Window{session_name: format!("{}:{}", project_name.clone(), i+1), name: k.as_str().unwrap().to_string(), root: root.clone()}));
                            commands.push(Command::SendKeys(SendKeys{target: format!("{}:{}", project_name, k.as_str().unwrap().to_string()).to_string(), exec: v.as_str().expect("Bad exec command").to_string()}));
                        }
                    }
                },
                &Yaml::String(ref s) => {
                    commands.push(Command::Window(Window{session_name: format!("{}:{}", project_name.clone(), i+1), name: s.clone(), root: root.clone()}))
                },
                &Yaml::Integer(ref s) => {
                    commands.push(Command::Window(Window{session_name: format!("{}:{}", project_name.clone(), i+1), name: s.to_string(), root: root.clone()}))
                },
                _ => panic!("Muxed config file formatting isn't recognized.")
            };
        };
    };

    commands.push(Command::KillWindow(KillWindow{name: tmp_window_name.clone()}));
    commands.push(Command::Attach(Attach{name: project_name.clone()}));
    commands
}

/// Pane matcher is for breaking apart the panes. Splitting windows when needed
/// and executing commands as needed.
fn pane_matcher(session: String, panes: &Yaml, root: Option<String>, window: String) -> Vec<Command> {
    let mut commands = vec!();
    let panes2 = panes["panes"].as_vec().expect("Something is wrong with panes.");

    for (i, pane) in panes2.iter().enumerate() {
        // For every pane, we need one less split.
        // ex. An existing window to become 2 panes, needs 1 split.
        if i < (panes2.len()-1) {
            commands.push(Command::Split(Split{target: format!("{}:{}.{}", session, window, i.to_string()).to_string(), root: root.clone()}));
        };
        // Execute given commands in each new pane after all splits are
        // complete.
        commands.push(Command::SendKeys(SendKeys{target: format!("{}:{}.{}", session, window, i).to_string(), exec: pane.as_str().expect("Bad exec command").to_string()}));
    };

    // After all panes are split select the layout for the window
    if panes["layout"].as_str().is_some() {
        let layout = panes["layout"].as_str().expect("Bad layout").to_string();
        commands.push(Command::Layout(Layout{target: format!("{}:{}", session, window).to_string(), layout: layout}));
    };

    commands
}

#[test]
pub fn windows_defined_as_array_has_6_commands() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    assert_eq!(main(&yaml, &"muxed".to_string()).len(), 6)
}

#[test]
pub fn windows_defined_as_array_has_1_session() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, &"muxed".to_string()).into_iter().filter(|x| match x {
        &Command::Session(_) => true,
        _ => false
    }).collect();

    assert_eq!(remains.len(), 1)
}

#[test]
pub fn windows_defined_as_array_has_3_windows() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, &"muxed".to_string()).into_iter().filter(|x| match x {
        &Command::Window(_) => true,
        _ => false
    }).collect();

    assert_eq!(remains.len(), 3)
}

#[test]
pub fn windows_defined_as_array_has_1_attach() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, &"muxed".to_string()).into_iter().filter(|x| match x {
        &Command::Attach(_) => true,
        _ => false
    }).collect();

    assert_eq!(remains.len(), 1)
}

#[test]
pub fn windows_with_integer_names() {
    let s = "---
windows: [1, 'vim', 3]
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    assert_eq!(main(&yaml, &"muxed".to_string()).len(), 6)
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
    let commands = main(&yaml, &"muxed".to_string());
    assert_eq!(commands.len(), 9)
}

#[test]
pub fn panes_array_has_8_commands() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml, &"muxed".to_string());
    assert_eq!(commands.len(), 8)
}

#[test]
pub fn panes_array_has_1_split() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";

    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, &"muxed".to_string()).into_iter().filter(|x| match x {
        &Command::Split(_) => true,
        _ => false
    }).collect();

    assert_eq!(remains.len(), 1)
}

#[test]
pub fn panes_array_has_1_layout() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";

    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, &"muxed".to_string()).into_iter().filter(|x| match x {
        &Command::Layout(_) => true,
        _ => false
    }).collect();

    assert_eq!(remains.len(), 1)
}

#[test]
pub fn panes_array_has_1_window() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";

    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, &"muxed".to_string()).into_iter().filter(|x| match x {
        &Command::Window(_) => true,
        _ => false
    }).collect();

    assert_eq!(remains.len(), 1)
}

#[test]
pub fn panes_array_has_1_session() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";

    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, &"muxed".to_string()).into_iter().filter(|x| match x {
        &Command::Session(_) => true,
        _ => false
    }).collect();

    assert_eq!(remains.len(), 1)
}

//#[test]
//pub fn panes_command_execs_array() {
//    let s = "---
//windows:
//  - editor:
//      layout: 'main-vertical'
//      panes: ['vim', 'guard']
//";
//    let yaml = YamlLoader::load_from_str(s).unwrap();
//    let remains: Vec<Command> = main(&yaml, &"muxed".to_string()).into_iter().filter(|x| match x {
//      &Command::SendKeys(_) => true,
//      _ => false
//    }).collect();
//
//    assert_eq!(remains.len(), 2);
//    assert_eq!(remains[0].exec, "vim");
//    assert_eq!(remains[1].exec, "guard")
//}
