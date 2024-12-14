// imports
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::os::unix::net::UnixStream;
use std::thread;

// constants
use light_control_app::common::constants::{SWITCH_ON, SWITCH_OFF, SOCKET_PATH, LIGHT_CONTROL_PATH};

fn main() {
    // 新しいプロセスとして light_control を起動
    let mut child = Command::new(LIGHT_CONTROL_PATH)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start light_control process");

    // メインスレッドでユーザー入力を待つ
    let handle = thread::spawn(move || {
        let mut input = String::new();
        loop {
            print!("Enter '1' to turn on, '2' to turn off, 'exit' to stop the process: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();
            if trimmed == "exit" {
                break;
            } else if trimmed == "1" {
                // プロセス間通信の例
                println!("Sending message to light_control process...");
                if let Ok(mut stream) = UnixStream::connect(SOCKET_PATH) {
                    stream.write_all(&[SWITCH_ON]).expect("Failed to send message");
                }
            } else if trimmed == "2" {
                // プロセス間通信の例
                println!("Sending message to light_control process...");
                if let Ok(mut stream) = UnixStream::connect(SOCKET_PATH) {
                    stream.write_all(&[SWITCH_OFF]).expect("Failed to send message");
                }
            }
            input.clear();
        }
    });

    // ユーザー入力スレッドが終了するのを待つ
    handle.join().unwrap();

    // プロセスを停止する
    child.kill().expect("Failed to kill light_control process");
    let _ = child.wait().expect("Failed to wait on child process");
}