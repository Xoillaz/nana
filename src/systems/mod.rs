mod collisions;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;
use crate::prelude::*;

// A stub awaits implement.
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .flush() // Guarentee all systems up to that point have finished before the next one runs.
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}
