use std::path::posix::Path;
use std::io::fs::PathExtensions;
use std::io::process::Command;

mod io;

static TEMPLATE: &'static str = include_str!("template.toml");

pub fn main(path: Path) {
    if !path.exists() {
 //       let filename = path.filename().unwrap();
//        File::create(path).write(modified_template(TEMPLATE, filename.to_string().as_slice()).as_bytes());
        try_or_err!(io::create(&path, ""), "Failed to create project file");
        //Err(_e) => println!("Failed to create project file {}", path.display()),

        match is_default_editor_set() {
            true  => io::open(&path),
            false => println!("Default editor is not set. Your config has been created and can be found in ~/.muxed/. Please define $EDITOR in your ~/.bashrc or similar file.")
        }
    } else {
        println!("Project already exists.");
    }
}

/// Run `which $EDITOR` to see if a default editor is defined on the system.
fn is_default_editor_set() -> bool {
  let output = match Command::new("which").arg("$EDITOR").output() {
      Ok(output) => output.output.to_string(),
      Err(e)     => fail!("failed to execute process: {}", e),
  };

  !output.is_empty()
}

fn modified_template(template: &str, project_name: &[u8]) -> String {
    let name = String::from_utf8(project_name.to_vec()).unwrap();
    template.replace("{file_name}", name.as_slice())
}

#[test]
fn populates_template_values() {
    let name   = "muxed projects".as_bytes();
    let value  = modified_template(TEMPLATE, name);
    let result = value.as_slice().contains("muxed project");
    assert!(result);
}

#[test]
fn removes_template_placeholders() {
    let name   = "muxed projects".as_bytes();
    let value  = modified_template(TEMPLATE, name);
    let result = !value.as_slice().contains("{file_name}");
    assert!(result);
}

//#[test]
//fn create_copies_the_template_file() {
//    let muxed_dir = &root::muxed_dir();
//    let path = &Path::new(format!("{}/{}.toml", muxed_dir.display(), random_name()));
//    let filename = project_filename(path);
//    create_project_file(path);
//    let data = File::open(path).read_to_end().unwrap();
//    let template_expectation = modified_template(TEMPLATE, filename.as_slice());
//    assert_eq!(data.as_slice(), template_expectation.as_bytes());
//
//    cleanup_dir(muxed_dir);
//}

//#[test]
//fn new_writes_file_to_muxed_dir() {
//    let name = random_name();
//    let muxed_dir = root::muxed_dir();
//    let path = &Path::new(format!("{}/{}", muxed_dir.display(), name));
//
//    println!("{}", path.display());
//
//    new(name.as_slice());
//    assert!(path.exists());
//
//    cleanup_dir(path);
//}
