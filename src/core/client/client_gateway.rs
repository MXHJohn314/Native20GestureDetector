struct ClientGateway {
    server_ipc: ServerIPC,
    client_ipc: ClientIPC,
}
impl ClientGateway {
    fn new(working_directory: &str) -> Self {
        match ServerIPC::new(working_directory, false) {
            Ok(server_ipc) => match ClientIPC::new(working_directory, true) {
                Ok(client_ipc) => Self {
                    server_ipc,
                    client_ipc
                },
                Err(e) => panic!("{:?}. Shutting down.", e)
            },
            Err(e) => panic!("{:?}. Shutting down.", e)
        }
    }
    fn request() {}
}
