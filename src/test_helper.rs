#[cfg(test)] use std::rand::random;

/// Test helper to standardize how random files and directories are generated.
#[cfg(test)]
pub fn random_name() -> String {
    format!("test_{}", random::<f64>())
}

