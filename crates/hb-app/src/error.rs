/// Command result type. The `String` error is serialised to the frontend as a
/// plain error message via Tauri's invoke system.
pub type CmdResult<T> = Result<T, String>;

pub fn cmd_err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}
