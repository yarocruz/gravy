fn main() {
    // gravy::clear_screen_to_color(0.0, 0.0, 1.0, 1.0);

    // let mut blue_amount = 0.0;

    let mut x_position = 200.0;
    let mut y_position = 30.0;

    gravy::set_event_handler(move |key| {
        let move_amount = 20.0;
        match key {
            gravy::Key::Left => x_position -= move_amount,
            gravy::Key::Right => x_position += move_amount,
            gravy::Key::Up => y_position += move_amount,
            gravy::Key::Down => y_position -= move_amount,
            gravy::Key::Space => {}
        }

        // blue_amount += 0.1;
        gravy::clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
        gravy::draw_rectangle(x_position, y_position, 100., 100.);
    })
}