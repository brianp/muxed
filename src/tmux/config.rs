//! The tmux config data we want

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_index: usize,
    pub pane_base_index: usize 
}

impl Config {
    pub fn from_string(options: String) -> Config {
        let lines = options.lines();
        let mut config: HashMap<&str, &str> = HashMap::new();

        for line in lines {
            let opt: Vec<&str> = line.split(" ").collect();
            config.insert(opt[0], opt[1]);
        }

        Config {
            base_index: usize::from_str(config.get("base-index").unwrap_or(&"0")).unwrap(),
            pane_base_index: usize::from_str(config.get("pane-base-index").unwrap_or(&"0")).unwrap()
        }
    }
}

#[test]
fn expect_base_index_0() {
    let output = "some-stuff false\nbase-index 0\nother-thing true".to_string();
    let config = Config::from_string(output);
    assert_eq!(config.base_index, 0)
}

#[test]
fn expect_base_index_5() {
    let output = "some-stuff false\nbase-index 5\nother-thing true".to_string();
    let config = Config::from_string(output);
    assert_eq!(config.base_index, 5)
}

#[test]
fn expect_missing_base_index_0() {
    let output = "some-stuff false".to_string();
    let config = Config::from_string(output);
    assert_eq!(config.base_index, 0)
}

#[test]
fn expect_pane_base_index_0() {
    let output = "some-stuff false\npane-base-index 0\nother-thing true".to_string();
    let config = Config::from_string(output);
    assert_eq!(config.pane_base_index, 0)
}

#[test]
fn expect_pane_base_index_5() {
    let output = "some-stuff false\npane-base-index 5\nother-thing true".to_string();
    let config = Config::from_string(output);
    assert_eq!(config.pane_base_index, 5)
}

#[test]
fn expect_missing_pane_base_index_0() {
    let output = "some-stuff false".to_string();
    let config = Config::from_string(output);
    assert_eq!(config.pane_base_index, 0)
}
