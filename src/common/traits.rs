use crate::common::constants::AppError;

trait Process {
    fn start(&self);
    fn stop(&self);
    fn get_thread_handle(&self) -> Option<&JoinHandle<()>>;
}

trait ThreadSpawner {
    fn spawn_thread(&self);
}

trait SocketListener {
    fn listen_socket(&self) -> Result<(), AppError>;
}