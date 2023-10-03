mod game;
pub mod cell;

use std::cell::RefCell;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::cell::Cell;
use crate::game::GameContext;

const GRID_SIZE: (u32, u32) = (64, 64);
const LINE_SIZE: u32 = 1;
const CELL_SIZE: (u32, u32) = (20, 20);
const WIN_SIZE: (u32, u32) = (CELL_SIZE.0 * GRID_SIZE.0 + LINE_SIZE * (GRID_SIZE.0 - 1),
                              CELL_SIZE.1 * GRID_SIZE.1 + LINE_SIZE * (GRID_SIZE.1 - 1));

const MAIN_LOOP_TIMEOUT_NANOS: u32 = 1_000_000_000u32 / 60;
const GAME_LOOP_TIMEOUT_NANOS: u32 = 50_000_000u32;

fn draw_cell(canvas: &mut WindowCanvas, cell: &RefCell<Cell>) {
    let mut cell = cell.borrow_mut();
    if !cell.changed {
        return;
    }
    if cell.is_dead() {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
    } else {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
    }

    canvas.fill_rect(Rect::new(
        (cell.x * CELL_SIZE.0 + LINE_SIZE * cell.x) as i32,
        (cell.y * CELL_SIZE.1 + LINE_SIZE * cell.y) as i32,
        CELL_SIZE.0,
        CELL_SIZE.1
    )).unwrap();

    cell.changed = false;
}

fn draw(canvas: &mut WindowCanvas, context: &GameContext) {
    for line in &context.cells {
        for cell in line {
            draw_cell(canvas, cell);
        }
    }
    canvas.present();
}

fn get_cell_coord_by_cursor(x: i32, y: i32) -> Option<(u32, u32)> {
    if x < 0 || y < 0 {
        return None;
    }

    let x= x as u32;
    let y = y as u32;

    let x_div_rem = x % (CELL_SIZE.0 + LINE_SIZE);
    if x_div_rem == 0 || x_div_rem == CELL_SIZE.0 {
        return None;
    }

    let y_div_rem = y % (CELL_SIZE.1 + LINE_SIZE);
    if y_div_rem == 0 || y_div_rem == CELL_SIZE.1 {
        return None;
    }

    let x_div = x / (CELL_SIZE.0 + LINE_SIZE);
    let y_div = y / (CELL_SIZE.1 + LINE_SIZE);

    Some((x_div, y_div))
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Life game", WIN_SIZE.0, WIN_SIZE.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let game_context = GameContext::new(GRID_SIZE.0, GRID_SIZE.1);

    draw(&mut canvas, &game_context);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut time = 0u32;

    let mut redraw = false;
    let mut freeze = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::MouseButtonDown {mouse_btn, x, y,..} => {
                    let mut revive = false;
                    if let MouseButton::Left = mouse_btn {
                        revive = true;
                    }

                    if let Some((cell_x, cell_y)) = get_cell_coord_by_cursor(x, y) {
                        game_context.set_living(cell_x, cell_y, revive);
                        redraw = true;
                    }
                    freeze = true;
                }
                Event::MouseButtonUp {..} => {
                    freeze = false;
                }
                Event::MouseMotion {mousestate, x, y, ..} => {
                    if mousestate.left() || mousestate.right() {
                        let revive = mousestate.left();

                        if let Some((cell_x, cell_y)) = get_cell_coord_by_cursor(x, y) {
                            game_context.set_living(cell_x, cell_y, revive);
                            redraw = true;
                        }
                    }
                }
                _ => {}
            }
        }


        if redraw {
            draw(&mut canvas, &game_context);
        }

        if time > GAME_LOOP_TIMEOUT_NANOS {
            if !freeze {
                game_context.tick();
                draw(&mut canvas, &game_context);
            }
            time -= GAME_LOOP_TIMEOUT_NANOS;
        } else {
            time += MAIN_LOOP_TIMEOUT_NANOS;
        };

        ::std::thread::sleep(Duration::new(0, MAIN_LOOP_TIMEOUT_NANOS));
    }
}
