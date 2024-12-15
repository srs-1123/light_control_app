// imports
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::os::unix::net::UnixStream;
use std::thread;

// constants
use light_control_app::common::constants::{SWITCH_ON, SWITCH_OFF, SOCKET_PATH, LIGHT_CONTROL_PATH, SENSOR_CONTROL_PATH};

// 定数配列としてプロセスのパスを管理
const PROCESS_PATHS: [&str; 2] = [SENSOR_CONTROL_PATH, LIGHT_CONTROL_PATH];

fn main() {
    // 新しいプロセスとして light_control を起動
    let mut child_processes = Vec::new();

    // 定数配列を使ってプロセスを順番に起動
    for &process_path in PROCESS_PATHS.iter() {
        println!("Starting process: {}", process_path);

        // プロセスを起動
        let child = Command::new(process_path)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start process");

        // 起動したプロセスをchild_processesベクタに格納
        child_processes.push(child);
    }
    
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

    // すべてのプロセスを終了させるためにkillし、終了を待つ
    for mut child in child_processes {
        // プロセスをkill
        child.kill().expect("Failed to kill process");

        // プロセスの終了を待つ
        let _ = child.wait().expect("Failed to wait on child process");
    }

    println!("All processes have been killed and completed.");
}