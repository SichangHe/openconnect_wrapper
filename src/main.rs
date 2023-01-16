use std::{
    ffi::OsStr,
    io::{stdin, Result, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{channel, Receiver, Sender},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

const WAIT: Duration = Duration::from_millis(300);

/// Use openconnect to connect to URL specified in `create_process`
/// and fill in information specified in `get_process_stdin` automatically.
///
/// Listen to stdin
/// and filter out input invalid to openconnect.
///
/// Press `r` + `enter` to reconnect.
fn main() -> Result<()> {
    let stdin = pipe_stdin();
    '_new_process: loop {
        let mut process = Process {
            child: create_process()?,
        };
        let mut process_stdin = get_process_stdin(&mut process.child)?;
        '_read_line: loop {
            sleep(WAIT);
            if process.child.try_wait()?.is_some() {
                eprintln!("[WRAPPER] openconnect already exited.");
                notify_death();
                break;
            }
            if check_pipe_death(&stdin) {
                eprintln!("[WRAPPER] StdIn closed, exiting.");
                break '_new_process;
            }
            let buf = match read_stdin_pipe(&stdin) {
                Some(b) => b,
                None => continue,
            };
            if buf == "r" {
                break;
            } else if buf.trim().chars().all(|c| c.is_numeric()) {
                process_stdin.write_all(buf.as_bytes())?;
                process_stdin.write_all(b"\n")?;
            } else {
                eprintln!("[WRAPPER] input `{buf}` is not numeric")
            }
        }
    }
    Ok(())
}

/// Wrapper for Child
struct Process {
    child: Child,
}

impl Drop for Process {
    fn drop(&mut self) {
        if self.child.try_wait().unwrap().is_some() {
            eprintln!("[WRAPPER] detected openconnect already exited.")
        } else {
            kill_process(self.child.id().to_string()).unwrap();
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

fn kill_process<S>(process_id: S) -> Result<()>
where
    S: AsRef<OsStr>,
{
    eprintln!("[WRAPPER] killing openconnect");
    let mut kill = Command::new("kill").arg(process_id).spawn()?;
    kill.wait()?;
    Ok(())
}

fn notify_death() {
    _ = Command::new("osascript")
        .args([
            "-e",
            r#"'display notification "openconnect died" with title "openconnect_wrapper"'"#,
        ])
        .spawn()
        .unwrap()
        .wait();
}

struct StdinPipe {
    rx: Receiver<String>,
    pipe: JoinHandle<()>,
}

fn pipe_stdin() -> StdinPipe {
    let (tx, rx) = channel();
    let pipe = spawn(move || spawn_stdin_pipe(tx));
    StdinPipe { rx, pipe }
}

fn spawn_stdin_pipe(tx: Sender<String>) {
    for maybe_line in stdin().lines() {
        let line = match maybe_line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("[WRAPPER] StdIn error: {e}.");
                continue;
            }
        };
        if let Err(e) = tx.send(line) {
            eprintln!("[WRAPPER] StdIn error: {e}.");
        }
    }
}

fn check_pipe_death(pipe: &StdinPipe) -> bool {
    pipe.pipe.is_finished()
}

fn read_stdin_pipe(pipe: &StdinPipe) -> Option<String> {
    pipe.rx.try_recv().ok()
}
