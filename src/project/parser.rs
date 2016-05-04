use yaml_rust::Yaml;
use command::{Command, Session, SendKeys, Split, Layout, Window, Window2, Attach};

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

        for window in windows.iter() {
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
        if i < (panes2.len()-1) {
            commands.push(Command::Split(Split{target: format!("{}:{}.{}", session, window, i.to_string()).to_string(), root: root.clone()}));
        };
        commands.push(Command::SendKeys(SendKeys{target: format!("{}:{}.{}", session, window, i).to_string(), exec: pane.as_str().expect("Bad exec command").to_string()}));
    };

    if panes["layout"].as_str().is_some() {
        let layout = panes["layout"].as_str().expect("Bad layout").to_string();
        commands.push(Command::Layout(Layout{target: format!("{}:{}", session, window).to_string(), layout: layout}));
    };

    commands
}

#[test]
pub fn windows_defined_as_array_has_4_commands() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    assert_eq!(main(&yaml, "muxed".to_string()).len(), 4)
}

#[test]
pub fn windows_defined_as_array_has_1_session() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, "muxed".to_string()).into_iter().filter(|x| match x {
      &Command::Session(_) => true,
      _ => false
    }).collect();

    assert_eq!(remains.len(), 1)
}

#[test]
pub fn windows_defined_as_array_has_2_windows() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, "muxed".to_string()).into_iter().filter(|x| match x {
      &Command::Window2(_) => true,
      _ => false
    }).collect();

    assert_eq!(remains.len(), 2)
}

#[test]
pub fn windows_defined_as_array_has_1_attach() {
    let s = "---
windows: ['cargo', 'vim', 'git']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, "muxed".to_string()).into_iter().filter(|x| match x {
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
    assert_eq!(main(&yaml, "muxed".to_string()).len(), 4)
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
    assert_eq!(commands.len(), 7)
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

    let first_window: Option<Session> = match commands[0].clone() {
        Command::Session(w) => Some(w),
        _                   => None
    };

    assert_eq!(first_window.unwrap().root.unwrap(), "~/.muxed".to_string())
}

#[test]
pub fn panes_array_has_7_commands() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";
    let yaml = YamlLoader::load_from_str(s).unwrap();
    let commands = main(&yaml, "muxed".to_string());
    println!("{:?}", commands);
    assert_eq!(commands.len(), 6)
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
    let remains: Vec<Command> = main(&yaml, "muxed".to_string()).into_iter().filter(|x| match x {
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
    let remains: Vec<Command> = main(&yaml, "muxed".to_string()).into_iter().filter(|x| match x {
      &Command::Layout(_) => true,
      _ => false
    }).collect();

    assert_eq!(remains.len(), 1)
}

#[test]
pub fn panes_array_has_no_window() {
    let s = "---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vim', 'guard']
";

    let yaml = YamlLoader::load_from_str(s).unwrap();
    let remains: Vec<Command> = main(&yaml, "muxed".to_string()).into_iter().filter(|x| match x {
      &Command::Window2(_) => true,
      _ => false
    }).collect();

    assert_eq!(remains.len(), 0)
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
    let remains: Vec<Command> = main(&yaml, "muxed".to_string()).into_iter().filter(|x| match x {
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
//    let remains: Vec<Command> = main(&yaml, "muxed".to_string()).into_iter().filter(|x| match x {
//      &Command::SendKeys(_) => true,
//      _ => false
//    }).collect();
//
//    assert_eq!(remains.len(), 2);
//    assert_eq!(remains[0].exec, "vim");
//    assert_eq!(remains[1].exec, "guard")
//}
