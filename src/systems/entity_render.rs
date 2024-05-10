use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // Query for all entities that have a point and render component.
    <(&Point, &Render)>::query()
        // Use the SubWorld to tranform the query into a iterator.
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // The map may include 4000 elements, leave some room in case that
    // changes add some user interface elements.
    draw_batch.submit(5000).expect("Batch error");
}
