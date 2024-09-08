#[derive(Debug, PartialEq, Clone, Eq)]
enum ServerState {
    Busy,
    Idle,
    Done,
}