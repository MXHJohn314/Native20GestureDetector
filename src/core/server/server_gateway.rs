struct ServerGateway {
    server_ipc: ServerIPC,
    client_ipc: ClientIPC,
}

/// The server side application uses the ServerGateway
impl ServerGateway {
    fn new(working_directory: &str) -> Self {
        match ServerIPC::new(working_directory, true) {
            Ok(server_ipc) => match ClientIPC::new(working_directory, false) {
                Ok(client_ipc) => Self {
                    server_ipc,
                    client_ipc
                },
                Err(e) => panic!("{:?}. Shutting down.", e)
            },
            Err(e) => panic!("{:?}. Shutting down.", e)
        }
    }
    fn start() {}
    fn stop() {}
}