use std::ffi::OsStr;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pid {
    pub pid: i32,
    pub pid_str: String,
}

impl Pid {
    pub fn new<S>(pid: S) -> Pid
        where S: Into<String> {

        let pid_s = pid.into();
        let pid_i = i32::from_str(pid_s.as_str()).unwrap();

        Pid {
            pid: pid_i,
            pid_str: pid_s,
        }
    }
}

impl fmt::Display for Pid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pid)
    }
}

impl AsRef<OsStr> for Pid {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(&self.pid_str)
    }
}
