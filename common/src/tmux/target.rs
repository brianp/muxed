use crate::error::CommonError;
use serde::{Deserialize, Serialize};

/// Represents a tmux target, which may include a session, window, and pane,
/// and provides a combined string representation suitable for tmux commands.
///
/// # Fields
/// - `session`: The tmux session name.
/// - `window`: An optional window index within the session.
/// - `pane`: An optional pane index within the window.
/// - `combined`: A string representation in the form `session`, `session:window`,
///   or `session:window.pane` as appropriate.
///
/// # Examples
/// Creating targets for different granularities:
/// ```rust
/// use common::tmux::Target;
///
/// let session = Target::new("mysession", None, None); // "mysession"
/// let window = Target::new("mysession", Some(2), None); // "mysession:2"
/// let pane = Target::new("mysession", Some(2), Some(1)); // "mysession:2.1"
/// ```
///
/// You can also extend a session-only target to include window or pane:
/// ```rust
/// use common::tmux::Target;
///
/// let t = Target::new("s", None, None);
/// let t = t.extend(1).unwrap(); // Now session "s", window 1
/// assert_eq!(t.combined, "s:1");
///
/// let t = t.extend(2).unwrap(); // Now session "s", window 1, pane 2
/// assert_eq!(t.combined, "s:1.2");
///
/// ```
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Target {
    pub session: String,
    pub window: Option<usize>,
    pub pane: Option<usize>,
    pub combined: String,
}

impl Target {
    pub fn new<S: Into<String> + ToString>(
        session: S,
        window: Option<usize>,
        pane: Option<usize>,
    ) -> Self {
        let mut combined = session.to_string();

        if let Some(ref window) = window {
            combined.push(':');
            combined.push_str(&window.to_string());
        }

        if let Some(ref pane) = pane {
            combined.push('.');
            combined.push_str(&pane.to_string());
        }

        Self {
            session: session.into(),
            window,
            pane,
            combined,
        }
    }

    pub fn extend(&self, value: usize) -> Result<Self, CommonError> {
        match (self.window, self.pane) {
            (w @ Some(_window), None) => Ok(Self::new(&self.session, w, Some(value))),
            (None, None) => Ok(Self::new(&self.session, Some(value), None)),
            _ => Err(CommonError::Target),
        }
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        self.combined == other.combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CommonError;

    #[test]
    fn test_new_session_only() {
        let target = Target::new("session1", None, None);
        assert_eq!(target.session, "session1");
        assert_eq!(target.window, None);
        assert_eq!(target.pane, None);
        assert_eq!(target.combined, "session1");
    }

    #[test]
    fn test_new_session_and_window() {
        let target = Target::new("mysession", Some(5), None);
        assert_eq!(target.session, "mysession");
        assert_eq!(target.window, Some(5));
        assert_eq!(target.pane, None);
        assert_eq!(target.combined, "mysession:5");
    }

    #[test]
    fn test_new_full_target() {
        let target = Target::new("abc", Some(3), Some(2));
        assert_eq!(target.session, "abc");
        assert_eq!(target.window, Some(3));
        assert_eq!(target.pane, Some(2));
        assert_eq!(target.combined, "abc:3.2");
    }

    #[test]
    fn test_extend_from_session_only() {
        let target = Target::new("x", None, None);
        let extended = target.extend(7).unwrap();
        assert_eq!(extended.session, "x");
        assert_eq!(extended.window, Some(7));
        assert_eq!(extended.pane, None);
        assert_eq!(extended.combined, "x:7");
    }

    #[test]
    fn test_extend_from_session_and_window() {
        let target = Target::new("baz", Some(2), None);
        let extended = target.extend(9).unwrap();
        assert_eq!(extended.session, "baz");
        assert_eq!(extended.window, Some(2));
        assert_eq!(extended.pane, Some(9));
        assert_eq!(extended.combined, "baz:2.9");
    }

    #[test]
    fn test_extend_from_full_target_fails() {
        let target = Target::new("zzz", Some(3), Some(5));
        let result = target.extend(8);
        assert!(matches!(result, Err(CommonError::Target)));
    }

    #[test]
    fn test_extend_from_window_and_pane_none_fails() {
        let target = Target::new("foo", None, Some(1));
        let result = target.extend(42);
        assert!(matches!(result, Err(CommonError::Target)));
    }
}
