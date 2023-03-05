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

    let count = 100;
    let mut automaton = gol::Gol::new(count, count);
    automaton.randomize();

    while !rl.window_should_close() {
        // Update phase
        update(&mut automaton, &rl);
        // Render phase
        let d = rl.begin_drawing(&thread);
        render(&automaton, d);
    }
}

enum Actions {
    Pause,
    Restart,
    Randomize,
    Clicked(i32, i32),
    Nothing,
}

fn handle_input(rl: &RaylibHandle) -> Actions {
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        return Actions::Pause;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_R) {
        return Actions::Restart;
    }
    if rl.is_key_pressed(KeyboardKey::KEY_A) {
        return Actions::Randomize;
    }
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        return Actions::Clicked(rl.get_mouse_x(), rl.get_mouse_y());
    }
    Actions::Nothing
}

fn update(automaton: &mut gol::Gol, rl: &RaylibHandle) {
    let mut mouse_pos = Vector2::new(0.0, 0.0);
    match handle_input(rl) {
        Actions::Pause => automaton.paused = !automaton.paused,
        Actions::Restart => automaton.restart(),
        Actions::Randomize => {
            automaton.randomize();
            automaton.paused = true;
        }
        Actions::Clicked(mouse_x, mouse_y) => {
            mouse_pos.x = mouse_x as f32;
            mouse_pos.y = mouse_y as f32;
        }
        Actions::Nothing => {}
    };
    if !automaton.paused {
        automaton.update();
    }
}

fn render(automaton: &gol::Gol, mut d: RaylibDrawHandle) {
    let min_side = if d.get_screen_width() > d.get_screen_height() {
        d.get_screen_height()
    } else {
        d.get_screen_width()
    } as f32;
    let cell_size = Vector2::new(
        // min_side / automaton.get_x_count() as f32,
        // min_side / automaton.get_y_count() as f32,
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
                Vector2::new(x as f32 * cell_size.x, y as f32 * cell_size.y),
                cell_size,
                color,
            );
        }
    }

    // Draw vertical line borders between cells
    for x in 1..automaton.get_x_count() {
        d.draw_line_ex(
            Vector2::new(x as f32 * cell_size.x, 0.0),
            Vector2::new(x as f32 * cell_size.x, d.get_screen_height() as f32),
            2.0,
            Color::DARKGRAY,
        );
    }

    // Draw horizontal line borders between cells
    for y in 1..automaton.get_y_count() {
        d.draw_line_ex(
            Vector2::new(0.0, y as f32 * cell_size.y),
            Vector2::new(d.get_screen_width() as f32, y as f32 * cell_size.y),
            2.0,
            Color::DARKGRAY,
        );
    }

    // Draw pause indicator if paused
    if automaton.paused {
        draw_pause(&mut d, Vector2::new(75.0, 75.0));
    }
}

pub fn draw_pause(d: &mut RaylibDrawHandle, pos: Vector2) {
    let rect_width = 14.0;
    d.draw_rectangle_v(
        pos,
        Vector2::new(rect_width, rect_width * 4.0),
        Color::new(186, 39, 32, 255),
    );
    d.draw_rectangle_v(
        Vector2::new(pos.x + (rect_width * 2.0), pos.y),
        Vector2::new(rect_width, rect_width * 4.0),
        Color::new(186, 39, 32, 255),
    );
}
