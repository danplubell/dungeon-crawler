#[derive(Copy, Clone, Debug, PartiaEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn
}