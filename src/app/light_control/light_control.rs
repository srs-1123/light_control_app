use crate::app::light_control::light_control_thread::LightControlThread;
use crate::common::constants::SOCKET_PATH;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::os::unix::net::UnixListener;
use std::io::{Read};

// 自分が作ったクレート
use crate::common::constants::AppError;

#[derive(Debug)]
pub enum LightControlError {
    SocketError(std::io::Error),
    MessageError(String),
}

pub struct LightControl {
    socket_path: String,
    light_control_thread: Arc<Mutex<LightControlThread>>,
    sender: Sender<u8>, // メッセージの型をu8に変更
}

impl LightControl {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let light_control_thread = LightControlThread::new(receiver);

        Self {
            socket_path: String::from(SOCKET_PATH),
            light_control_thread: Arc::new(Mutex::new(light_control_thread)),
            sender,
        }
    }

    pub fn start(&self) {
        let light_control_thread = Arc::clone(&self.light_control_thread);
        thread::spawn(move || {
            let light_control_thread = light_control_thread.lock().unwrap();
            light_control_thread.run();
        });

        if let Err(e) = self.listen_socket() {
            eprintln!("Socket listener error: {:?}", e);
        }
    }

    fn listen_socket(&self) -> Result<(), AppError> {
        let _ = std::fs::remove_file(&self.socket_path);
        let listener = UnixListener::bind(&self.socket_path).map_err(AppError::SocketError)?;

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            if size > 0 {
                                let message = buffer[0]; // 最初のバイトをu8として送信
                                // TODO: メッセージがプロセス終了なら、stopメソッドを実行
                                println!("Received message: {}", message);
                                // メッセージを送信
                                self.sender.send(message).map_err(|e| AppError::MessageError(e.to_string()))?;
                            }
                        }
                        Err(e) => eprintln!("Failed to read from socket: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Failed to accept connection: {:?}", e),
            }
        }
        Ok(())
    }
}