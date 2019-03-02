use std::process::Command;
use std::fmt;
use tmux::pane::pid::Pid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Process {
    pub process: String,
}

impl Process {
    pub fn new<S>(process: S) -> Process
        where S: Into<String> {
            Process {
                process: process.into(),
            }
    }

    pub fn process_string_from(pid: Pid) -> Result<String, String> {
        // we could cat /proc/pid/cmdline instead of calling pgrep
        let output = try!(Command::new("pgrep")
            .arg("-lf")
            .arg("-P")
            .arg(pid)
            .output()
            .map_err(|e| format!("We couldn't find the process for that pane: {}", e)));

        let read = String::from_utf8_lossy(&output.stdout);
        let process_string = match read.lines().next() {
            Some(x) => Process::strip_pid(x),
            None    => return Err(String::from("No process found"))
        };

        Ok(process_string)
    }

    fn strip_pid<T>(line: T) -> String
        where T: Into<String> + Clone {
          let temp = line.into().clone();
          temp.split_whitespace().skip(1).collect::<Vec<&str>>().join(" ")
    }
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.process)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expect_to_return_without_first_element() {
        let line = "85909 vim .";
        let process = Process::strip_pid(line);
        assert_eq!(process, "vim .")
    }
}
