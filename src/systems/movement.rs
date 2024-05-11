use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    intent_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(intent_move.destination) {
        // It's safer and more efficient to use commands to generate new
        // components, rather thatn directly modifying the old ones.
        commands.add_component(intent_move.entity, intent_move.destination);

        if ecs
            .entry_ref(intent_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(intent_move.destination);
        }
    }
    commands.remove(*entity);
}
