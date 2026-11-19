fn apply_brightness(unit: (), game: &mut Game<'_>) {
    if game.on("fullbright") && let Some(renderer) = game.renderer.as_mut() {
        renderer.fullbright = fullbright::active_for_render();
    }
    unit
}
