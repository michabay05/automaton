mod gol;

use raylib::prelude::*;

const WIN_SIZE: Vector2 = Vector2::new(1000.0, 700.0);

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIN_SIZE.x as i32, WIN_SIZE.y as i32)
        .title("Cellular Automaton")
        .resizable()
        .msaa_4x()
        .vsync()
        .build();

    let mut automaton = gol::Gol::new(100, 100);

    while !rl.window_should_close() {
        // Update phase
        update(&mut automaton);
        // Render phase
        let d = rl.begin_drawing(&thread);
        render(&automaton, d);
    }
}

fn update(automaton: &mut gol::Gol) {}

fn render(automaton: &gol::Gol, mut d: RaylibDrawHandle) {
    let cell_size = Vector2::new(
        d.get_screen_width() as f32 / automaton.get_x_count() as f32,
        d.get_screen_height() as f32 / automaton.get_y_count() as f32,
    );
    const BACKGROUND_COLOR: Color = Color::new(20, 20, 20, 255);
    d.clear_background(BACKGROUND_COLOR);
    const ALIVE_COLOR: Color = Color::BEIGE;
    const DEAD_COLOR: Color = BACKGROUND_COLOR;
    let mut color = Color::WHITE;
    // Draw cells
    for x in 0..automaton.get_x_count() {
        for y in 0..automaton.get_y_count() {
            color = match automaton.is_alive(x, y) {
                true => ALIVE_COLOR,
                false => DEAD_COLOR,
            };
            d.draw_rectangle_v(
                Vector2::new(
                    (x * cell_size.x as usize) as f32,
                    (y * cell_size.y as usize) as f32,
                ),
                cell_size,
                color,
            );
        }
    }

    // Draw vertical line borders between cells
    for x in 1..automaton.get_x_count() {
        d.draw_line_ex(
            Vector2::new((x * cell_size.x as usize) as f32, 0.0),
            Vector2::new(
                (x * cell_size.x as usize) as f32,
                d.get_screen_height() as f32,
            ),
            2.0,
            Color::DARKGRAY,
        );
    }

    // Draw horizontal line borders between cells
    for y in 1..automaton.get_y_count() {
        d.draw_line_ex(
            Vector2::new(0.0, (y * cell_size.y as usize) as f32),
            Vector2::new(
                d.get_screen_width() as f32,
                (y * cell_size.y as usize) as f32,
            ),
            2.0,
            Color::DARKGRAY,
        );
    }
}
