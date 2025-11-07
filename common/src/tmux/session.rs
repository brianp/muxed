use crate::tmux::pane::Pane;
use crate::tmux::window::Window;
use crate::tmux::{Config, Pre, Target};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Session {
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre: Option<Pre>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_window: Option<Pre>,
    pub root: Option<PathBuf>,
    pub windows: Vec<Window>,
    pub target: Option<Target>,
    pub daemonize: Option<bool>,
    pub config: Option<Config>,
}

impl Session {
    /// Returns an iterator over all windows and panes in the session,
    /// yielding immutable references in a well-defined sequence.
    ///
    /// This iterator yields each window (as a [`NodeRef::Window`]), followed
    /// by each pane in that window (as [`NodeRef::Pane`]), traversing the
    /// entire session tree in a flat, linear order.
    ///
    /// This custom iterator is unique in that it handles a hierarchical
    /// (window → pane) data structure seamlessly in a single pass, abstracting
    /// over the nested collections to present a unified sequence to the caller.
    pub fn iter(&self) -> SessionIter<'_> {
        SessionIter::new(self)
    }

    /// Returns a mutable iterator over all windows and panes in the session,
    /// yielding exclusive references in a flat traversal order.
    ///
    /// The mutable iterator is implemented using raw pointers to safely allow
    /// mutable access to each element without running afoul of Rust's strict
    /// borrowing rules. This enables traversing and mutating each window and
    /// pane in the session, one at a time, with safety guaranteed by the
    /// iterator's logic.
    ///
    /// This approach is unique because it enables in-place mutation of deeply
    /// nested items (panes), while respecting borrowing invariants,
    /// even as new mutable references are created sequentially.
    pub fn iter_mut(&mut self) -> SessionIterMut<'_> {
        SessionIterMut::new(self)
    }

    pub fn pre(&self) -> Option<&Pre> {
        self.pre.as_ref()
    }

    pub fn pre_window(&self) -> Option<&Pre> {
        self.pre.as_ref()
    }
}

#[derive(Debug)]
pub enum NodeRef<'a> {
    Window {
        index: usize,
        window: &'a Window,
    },
    Pane {
        window_index: usize,
        pane_index: usize,
        pane: &'a Pane,
    },
}

// Custom iterator over Session
pub struct SessionIter<'a> {
    windows: std::slice::Iter<'a, Window>,
    current_panes: Option<(usize, std::slice::Iter<'a, Pane>, usize)>, // (window_index, panes_iter, pane_index)
    window_index: usize,
}

impl<'a> SessionIter<'a> {
    pub fn new(session: &'a Session) -> Self {
        SessionIter {
            windows: session.windows.iter(),
            current_panes: None,
            window_index: 0,
        }
    }
}

impl<'a> Iterator for SessionIter<'a> {
    type Item = NodeRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // If we’re currently inside a window’s panes
        if let Some((win_idx, ref mut panes, ref mut pane_idx)) = self.current_panes {
            if let Some(pane) = panes.next() {
                let result = NodeRef::Pane {
                    window_index: win_idx,
                    pane_index: *pane_idx,
                    pane,
                };
                *pane_idx += 1;
                return Some(result);
            }
            // finished with this window’s panes
            self.current_panes = None;
        }

        // Move to next window
        if let Some(window) = self.windows.next() {
            let current_idx = self.window_index;
            self.window_index += 1;

            // initialize pane iterator for this window
            self.current_panes = Some((current_idx, window.panes.iter(), 0));

            return Some(NodeRef::Window {
                index: current_idx,
                window,
            });
        }

        None
    }
}

pub enum NodeMut<'a> {
    Window {
        index: usize,
        window: &'a mut Window,
    },
    Pane {
        window_index: usize,
        pane_index: usize,
        pane: &'a mut Pane,
    },
}

pub struct SessionIterMut<'a> {
    session: *mut Session, // raw pointer to sidestep borrow conflicts
    window_index: usize,
    pane_index: Option<usize>,
    len: usize,
    _marker: std::marker::PhantomData<&'a mut Session>,
}

impl<'a> SessionIterMut<'a> {
    pub fn new(session: &'a mut Session) -> Self {
        let len = session.windows.len();
        Self {
            session,
            window_index: 0,
            pane_index: None,
            len,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a> Iterator for SessionIterMut<'a> {
    type Item = NodeMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: only one &mut is created at a time, and never reused across iterations.
        let session = unsafe { &mut *self.session };

        // currently iterating panes
        if let Some(pane_idx) = self.pane_index {
            let num_panes = session.windows[self.window_index].panes.len();

            if pane_idx < num_panes {
                let pane = &mut session.windows[self.window_index].panes[pane_idx];
                self.pane_index = Some(pane_idx + 1);
                return Some(NodeMut::Pane {
                    window_index: self.window_index,
                    pane_index: pane_idx,
                    pane,
                });
            } else {
                // finished panes for this window
                self.pane_index = None;
                self.window_index += 1;
            }
        }

        // next window
        if self.window_index < self.len {
            let window = &mut session.windows[self.window_index];
            self.pane_index = Some(0);
            return Some(NodeMut::Window {
                index: self.window_index,
                window,
            });
        }

        None
    }
}

#[test]
fn test_various_windows() {
    let samples = [
        r#"
        windows: ['cargo', 'vim', 'git']
        "#,
        r#"
        windows: [1, 'vim', 3]
        "#,
        r#"
        windows:
          - cargo: ''
          - vim: ''
          - git: ''
        "#,
        r#"
        windows:
          - editor:
        "#,
        r#"
        pre_window:
         - 'ls'
         - 'ls'
        windows:
          - editor:
        "#,
    ];

    for yaml in samples {
        let session: Result<Session, _> = serde_saphyr::from_str(yaml);
        assert!(session.is_ok(), "failed on:\n{yaml}\n");
    }

    // error case
    let bad = r#"
    windows:
      - : ls
    "#;
    let session: Result<Session, _> = serde_saphyr::from_str(bad);
    assert!(session.is_err(), "failed on:\n{bad}\n");
}
