struct GestureDetectorService {
    gateway: ServerGateway,
}

impl GestureDetectorService {
    fn new(working_directory: &str) -> Self {
        let server_gateway = ServerGateway::new(working_directory);
        Self { gateway: server_gateway }
    }

    #[no_mangle]
    pub extern "C" 
    fn start(&mut self) {
        let HELLO_WORLD: i32 = "Hi".chars()
        .map(|x| x as u8)
        .fold(0, |acc, x| acc + x as i32);
        loop {
            match self.gateway.client_ipc.read_client_command() {
                ClientCommand::Ping => {
                    self.gateway.server_ipc.write_server_state(ServerState::Busy);
                    self.gateway.server_ipc.write_server_result(HELLO_WORLD);
                    self.gateway.server_ipc.write_server_state(ServerState::Done);
                }
                ClientCommand::DoWork => {
                    self.gateway.server_ipc.write_server_state(ServerState::Busy);
                    let swipe_direction = self.gateway.client_ipc.read_fling_swipe_direction();
                    let duration = self.gateway.client_ipc.read_duration();
                    let points = self.gateway.client_ipc.read_points();
                    let result = self.calculate_result(swipe_direction, duration, points);
                    self.gateway.server_ipc.write_server_result(result);
                    self.gateway.server_ipc.write_server_state(ServerState::Done);
                }
                ClientCommand::ClearState => {
                    self.gateway.server_ipc.write_server_state(ServerState::Idle);
                }
            }
        }
    }
    fn calculate_result(&self, swipe_direction: Option<DirectionResult>, duration: i32, points: [Point; 2000]) -> i32 {
        // Implement your calculation logic here
        // Return a random integer as a placeholder
        rand::random::<i32>()
    }

    #[no_mangle]
    pub extern "C" 
    fn stop(&mut self) {
        self.gateway.server_ipc.write_server_state(ServerState::Idle);
    }
}
