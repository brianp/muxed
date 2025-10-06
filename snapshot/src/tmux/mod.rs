pub mod pane;
pub mod session;
pub mod window;

use self::pane::Pane;
use self::session::Session;
use self::window::Window;

pub fn inspect(name: &str) -> Result<Session, String> {
    let windows = windows_for(name)?;
    let windows = windows
        .into_iter()
        .map(|w| Window::from_window(panes_for(name, &w).unwrap(), w))
        .collect();

    Ok(Session::new(name, windows))
}

fn windows_for(target: &str) -> Result<Vec<Window>, String> {
    let err = format!("\u{1F613} The session {} was not found.", target);
    let output = Window::window_list(target).map_err(|e| format!("{} - {}", err, e))?;

    let windows = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(Window::from_line)
        .collect::<Vec<_>>();

    if windows.is_empty() {
        return Err(err);
    }

    Ok(windows)
}

fn panes_for(session_name: &str, w: &Window) -> Result<Vec<Pane>, String> {
    let target = format!("{}:{}", &session_name, &w.name);
    let output = Pane::pane_list(&target)
        .map_err(|e| format!("We couldn't find panes for the {} window - {}", &target, e))?;

    let panes = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(Pane::from_line)
        .collect::<Vec<_>>();

    Ok(panes)
}
