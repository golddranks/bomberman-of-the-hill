use bomber_lib::{
    self,
    world::{Direction, Tile, World},
    Action, Player,
};
use bomber_macro::wasm_player;

/// To build a `wasm player`, all that's needed is to implement the
/// `Player` trait, which defines how the player interacts with the
/// world, and to mark the struct with the `wasm_player` attribute,
/// which exposes the `wasm` exports the game expects to hot-swap
/// the player in.
#[wasm_player]
struct Fool {
    choice: usize,
}

impl Player for Fool {
    fn spawn() -> Self {
        Self { choice: 0 }
    }

    fn act(&mut self, world: &impl World) -> Action {
        // A fool just ignores the world and travels north! Or somewhere
        // close to north! What's the worst that could happen?
        let possible_moves = [Direction::North, Direction::NorthWest, Direction::NorthEast];
        let chosen_move = possible_moves[self.choice];
        self.choice = (self.choice + 1) % possible_moves.len();
        // Maybe not if it's lava though... That would be bad! Best rethink it
        let chosen_move = if world.inspect(chosen_move) == Tile::Lava {
            possible_moves[self.choice]
        } else {
            chosen_move
        };
        Action::Move(chosen_move)
    }
}
