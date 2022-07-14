use std::{
    ffi::OsStr,
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
    let stdin = stdin();
    '_new_process: loop {
        let mut process = Process {
            child: create_process()?,
        };
        let mut process_stdin = get_process_stdin(&mut process.child)?;
        '_read_line: loop {
            let mut buf = String::new();
            stdin.read_line(&mut buf)?;
            if buf == "r\n" {
                break;
            } else if buf.trim().chars().all(|c| c.is_numeric()) {
                process_stdin.write_all(buf.as_bytes())?;
            } else {
                eprintln!("[WRAPPER] input `{buf}` is not numeric")
            }
        }
    }
}

/// Wrapper for Child
struct Process {
    child: Child,
}

impl Drop for Process {
    fn drop(&mut self) {
        kill_process(self.child.id().to_string()).unwrap();
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

fn kill_process<S>(process_id: S) -> Result<()>
where
    S: AsRef<OsStr>,
{
    eprintln!("[WRAPPER] killing openconnect");
    let mut kill = Command::new("kill").arg(process_id).spawn()?;
    kill.wait()?;
    Ok(())
}
