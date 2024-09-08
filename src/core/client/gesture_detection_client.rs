struct GestureDetectionClient {
    gateway: ClientGateway,
}

impl GestureDetectionClient {
    fn new(working_directory: &str) -> Self {
        let client_gateway = ClientGateway::new(working_directory);
        Self { gateway: client_gateway }
    }

    #[no_mangle]
    pub extern "C" 
    fn connect(&mut self) -> i32 {
        // Check if the necessary files exist
        let dir_gen_result = generate_direcectory("");
        if dir_gen_result.is_err() {
            panic!("{:?}", dir_gen_result.err());
        }

        // Check server state
        let server_state = self.gateway.server_ipc.read_server_state();
        while server_state != ServerState::Idle {
            // Wait for the server to be idle, sleep
            std::thread::sleep(std::time::Duration::from_secs(3));
        }

        // Connection successful
        0 // Exit code 0: Success
    }

    #[no_mangle]
    pub extern "C" 
    fn disconnect(&mut self) {
        self.gateway.server_ipc.write_server_state(ServerState::Idle);
    }

    #[no_mangle]
    pub extern "C" 
    fn do_work(&mut self, fling_swipe_direction: i32, duration: i64, points: [Point;2000]) -> i32 {
        self.gateway.client_ipc.write_fling_swipe_direction(Some(DirectionResult::Drag(SwipeDirection::East)));
        self.gateway.client_ipc.write_duration(100);
        self.gateway.client_ipc.write_points([Point::new(1.0, 2.0); 2000]);
        self.gateway.client_ipc.write_client_command(ClientCommand::DoWork);
        // Wait for server to finish processing
        loop {
            let server_state = self.gateway.server_ipc.read_server_state();
            if server_state == ServerState::Done {
                break;
            }
        }
        // Read the result from the server
        let result = self.gateway.server_ipc.read_server_result();
        self.gateway.client_ipc.write_client_command(ClientCommand::ClearState);
        result
    }
}
