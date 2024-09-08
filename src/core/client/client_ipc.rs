
struct ClientIPC{
    ipc: IPCProtocol,
}

impl ClientIPC {
    fn new(working_directory: &str, is_client: bool) -> io::Result<Self> {
        let ipc = IPCProtocol::new(working_directory, is_client, SERVER_SIZE)?;
        Ok(Self { ipc })
    }

    pub fn read_client_command(&self) -> ClientCommand {
        self.ipc.read_client_struct().client_state
    }
    pub fn write_client_command(&self, client_command: ClientCommand) {
        self.ipc.write_client_struct(&ClientStruct {
            client_state: client_command,
            swipe_direction: None,
            duration: 0,
            points: [Point::new(0.0, 0.0); 2000],
        });
    }

    pub fn read_fling_swipe_direction(&self) -> Option<DirectionResult> {
        self.ipc.read_client_struct().swipe_direction
    }
    pub fn write_fling_swipe_direction(&self, swipe_direction: Option<DirectionResult>) {
        self.ipc.write_client_struct(&ClientStruct {
            client_state: ClientCommand::Ping,
            swipe_direction,
            duration: 0,
            points: [Point::new(0.0, 0.0); 2000],
        });
    }

    pub fn read_duration(&self) -> i32 {
        self.ipc.read_client_struct().duration
    }
    pub fn write_duration(&self, duration: i32) {
        self.ipc.write_client_struct(&ClientStruct {
            client_state: ClientCommand::Ping,
            swipe_direction: None,
            duration,
            points: [Point::new(0.0, 0.0); 2000],
        });
    }

    pub fn read_points(&self) -> [Point; 2000] {
        self.ipc.read_client_struct().points
    }
    pub fn write_points(&self, points: [Point; 2000]) {
        self.ipc.write_client_struct(&ClientStruct {
            client_state: ClientCommand::Ping,
            swipe_direction: None,
            duration: 0,
            points,
        });
    }
}
