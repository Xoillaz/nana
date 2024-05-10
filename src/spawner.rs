use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    // Creates a new Entity composed of the listed components.
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
