use crate::error::CommonError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
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
