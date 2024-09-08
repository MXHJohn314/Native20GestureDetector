#[derive(Debug, PartialEq, Clone, Eq)]
struct ClientStruct {
    client_state: ClientCommand,
    swipe_direction: Option<DirectionResult>,
    duration: i32,
    points: [Point; 2000],
}