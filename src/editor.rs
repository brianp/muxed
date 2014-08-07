use std::io::process::Command;
use libc::funcs::c95::stdlib::system;

pub fn new(_name: &str) {
  println!("editor");
}

/// Run `which $EDITOR` to see if a default editor is defined on the system.
pub fn default_editor_set() -> bool {
  let output = match Command::new("which").arg("$EDITOR").output() {
      Ok(output) => output.output.to_string(),
      Err(e) => fail!("failed to execute process: {}", e),
  };

  !output.is_empty()
}

/// Open a project file with the default editor. Uses C directly to interact
/// with the system. This method is overloaded below for the test config to not
/// execture during testing.
#[cfg(not(test))] pub fn open_project_file(path: &Path) {
    let method = format!("$EDITOR {}", path.display()).to_c_str();
    unsafe { system(method.unwrap()); };
}

/// Overloaded method for use in testing. Doesn't do anything at all.
#[cfg(test)] pub fn open_project_file(_path: &Path) { }
