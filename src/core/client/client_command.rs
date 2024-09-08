#[derive(Debug, PartialEq, Clone, Eq)]
enum ClientCommand {
    Ping,
    DoWork,
    ClearState,
}
