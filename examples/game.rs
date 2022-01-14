fn main() {
    // gravy::clear_screen_to_color(0.0, 0.0, 1.0, 1.0);

    // let mut blue_amount = 0.0;

    gravy::set_event_handler(move |key| match key {
        gravy::Key::Left => gravy::clear_screen_to_color(1.0, 0.0, 0.0, 1.0),
        gravy::Key::Right => gravy::clear_screen_to_color(0.0, 1.0, 0.0, 1.0),
        gravy::Key::Up => gravy::clear_screen_to_color(0.0, 0.0, 1.0, 1.0),
        gravy::Key::Down => gravy::clear_screen_to_color(0.0, 1.0, 1.0, 1.0),
        gravy::Key::Space => gravy::clear_screen_to_color(1.0, 1.0, 0.0, 1.0)
        // blue_amount += 0.1;
        // gravy::clear_screen_to_color(0.0, 0.0, blue_amount, 1.0);
    })
}