use std::io::fs::File;
use project::{TEMPLATE,modified_template};
use libc::funcs::c95::stdlib::system;

/// Copy and create the new project file from a template.
pub fn create(path: &Path) {
    let filename = path.filename().unwrap();
    match File::create(path).write(modified_template(TEMPLATE, filename.to_string().as_slice()).as_bytes()) {
        Ok(())  => (),
        Err(_e) => println!("Failed to create project {}", filename),
    }
}

/// Open a project file with the default editor. Uses C directly to interact
/// with the system. This method is overloaded below for the test config to not
/// execture during testing.
#[cfg(not(test))] pub fn open(path: &Path) {
    let method = format!("$EDITOR {}", path.display()).to_c_str();
    unsafe { system(method.unwrap()); };
}

/// Overloaded method for use in testing. Doesn't do anything at all.
#[cfg(test)] pub fn open(_path: &Path) { }
