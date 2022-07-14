use std::{
    io::{stdin, Result, Write},
    process::{Child, ChildStdin, Command, Stdio},
};

/// Use openconnect to connect to URL specified in `create_process`
/// and fill in information specified in `get_process_stdin` automatically.
///
/// Listen to stdin
/// and filter out input invalid to openconnect.
///
/// Press `r` + `enter` to reconnect.
fn main() -> Result<()> {
    let mut process = create_process()?;
    let mut process_stdin = get_process_stdin(&mut process)?;
    let stdin = stdin();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        if buf == "r\n" {
            process.kill()?;
            process = create_process()?;
            process_stdin = get_process_stdin(&mut process)?;
        } else if buf.trim().chars().all(|c| c.is_numeric()) {
            process_stdin.write_all(buf.as_bytes())?;
        } else {
            eprintln!("[WRAPPER] input `{buf}` is not numeric")
        }
    }
}

/// Run `openconnect <URL_TO_CONNECT_TO>`.
fn create_process() -> Result<Child> {
    Command::new("openconnect")
        .arg("<URL_TO_CONNECT_TO>")
        .stdin(Stdio::piped())
        .spawn()
}

/// Fill in <GROUP>, <USERNAME>, and <PASSWORD> to openconnect's stdin.
fn get_process_stdin(process: &mut Child) -> Result<ChildStdin> {
    let mut process_stdin = process.stdin.take().unwrap();
    process_stdin.write_all(b"<GROUP>\n<USERNAME>\n<PASSWORD>\n")?;
    Ok(process_stdin)
}
