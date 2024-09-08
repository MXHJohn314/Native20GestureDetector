struct ServerIPC {
    ipc: IPCProtocol,
 }

impl ServerIPC {
    fn new(working_directory: &str, is_server: bool) -> io::Result<Self> {
        let ipc = IPCProtocol::new(working_directory, is_server, SERVER_SIZE)?;
        Ok(Self { ipc })
    }

    pub fn write_server_state(&self, server_state: ServerState) {
        self.ipc.write_server_struct(&ServerStruct {
            server_state,
            result: 0,
        });
    }
    pub fn write_server_result(&self, result: i32) {
        self.ipc.write_server_struct(&ServerStruct {
            server_state: ServerState::Idle,
            result,
        });
    }

    pub fn read_server_state(&self) -> ServerState {
        self.ipc.read_server_struct().server_state
    }

    pub fn read_server_result(&self) -> i32 {
        self.ipc.read_server_struct().result
    }
}