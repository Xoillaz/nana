mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 4;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 4;
    // Modules are organized as a tree, crate:: means accesses the root.
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1) // Skip the first room.
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new() // Create a generic terminal and specify attributes directly.
        .with_title("nana")
        .with_fps_cap(30.0) // Track game speed, tells OS it can rest between frames.
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) // The size of subsequent consoles.
        .with_tile_dimensions(32, 32) // The size of each character in the font file.
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Add a console using attributes specified.
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png") // Add a second console to show transparency.
        .build()?;

    main_loop(context, State::new())
}
