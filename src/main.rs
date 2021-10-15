use std::cmp::{min, max};
use std::vec::Vec;
use macroquad::prelude::*;


const SQUARE_SIZE: f32 = 4.0;
const HEADER_HEIGHT: f32 = 20.0;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point<T> {
    x: T,
    y: T,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

struct Game {
    generation: u32,
    width: usize,
    height: usize,
    cells: Vec<Vec<CellState>>,
    user: Point<usize>,
}

impl Game {
    fn new(width: usize, height: usize) -> Game {
        Game {
            generation: 0,
            width,
            height,
            cells: vec![vec![CellState::Dead; height]; width],
            user: Point {x: 0, y: 0},
        }
    }

    fn randomize_cells(&mut self) -> () {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                self.cells[x][y] = if rand::gen_range(0, 5) <= 1 {
                    CellState::Alive
                } else {
                    CellState::Dead
                }
            }
        }
        self.generation = 0;
    }

    fn clear_cells(&mut self) -> () {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                self.cells[x][y] = CellState::Dead;
            }
        }
        self.generation = 0;
    }

    fn move_user(&mut self, dx: i32, dy: i32) -> () {
        self.user.x = min(max(self.user.x as i32 + dx, 0), self.width as i32 - 1) as usize;
        self.user.y = min(max(self.user.y as i32 + dy, 0), self.height as i32 - 1) as usize;
    }

    fn reverse_by_user(&mut self) -> () {
        self.cells[self.user.x][self.user.y] = match self.cells[self.user.x][self.user.y] {
            CellState::Alive => CellState::Dead,
            CellState::Dead => CellState::Alive,
        }
    }

    fn draw(&self, offset: Point<f32>) -> () {
        // header
        draw_text(&format!("S: start/stop, N: next, R: randamize, C: clear, generation: {}", self.generation), 5.0, 15.0, 20.0, DARKGRAY);

        // cells
        for y in 0..self.height {
            for x in 0..self.width {
                match self.cells[x][y] {
                    CellState::Alive => {
                        draw_rectangle(x as f32 * SQUARE_SIZE + offset.x, y as f32 * SQUARE_SIZE + offset.y, SQUARE_SIZE, SQUARE_SIZE, BLACK);
                    },
                    CellState::Dead => {
                        draw_rectangle(x as f32 * SQUARE_SIZE + offset.x, y as f32 * SQUARE_SIZE + offset.y, SQUARE_SIZE, SQUARE_SIZE, WHITE);
                    },
                };
            }
        }

        // user
        let x1: f32 = self.user.x as f32 * SQUARE_SIZE + offset.x;
        let x2: f32 = (self.user.x + 1) as f32 * SQUARE_SIZE + offset.x;
        let y1: f32 = self.user.y as f32 * SQUARE_SIZE + offset.y;
        let y2: f32 = (self.user.y + 1) as f32 * SQUARE_SIZE + offset.y;
        draw_line(x1, y1, x2, y1, 1.0, GREEN);
        draw_line(x1, y1, x1, y2, 1.0, GREEN);
        draw_line(x2, y1, x2, y2, 1.0, GREEN);
        draw_line(x1, y2, x2, y2, 1.0, GREEN);
    }

    fn update(&mut self) -> () {
        let mut next_cells = vec![vec![CellState::Dead; self.height]; self.width];
        for y in 0..self.height as i16 {
            for x in 0..self.width as i16 {
                let mut alive_count: u16 = 0;
                for dy in -1..=1 as i16 {
                    if y+dy < 0 || self.height as i16 <= y+dy { continue; }  // はみ出してないか
                    for dx in -1..=1 as i16 {
                        if x+dx < 0 || self.width as i16 <= x+dx { continue; }  // はみ出してないか
                        if dx == 0 && dy == 0 { continue; }  // 同じマス
                        if self.cells[(x+dx) as usize][(y+dy) as usize] == CellState::Alive {
                            alive_count += 1;
                        }
                    }
                }

                next_cells[x as usize][y as usize] = match (self.cells[x as usize][y as usize], alive_count) {
                    (CellState::Alive, 2) | (CellState::Alive, 3) => CellState::Alive,  // 生存
                    (CellState::Alive, _) => CellState::Dead,  // 過疎 or 過密
                    (CellState::Dead, 3) => CellState::Alive,  // 誕生
                    (CellState::Dead, _) => CellState::Dead, // 誕生しない
                }
            }
        }
        self.cells = next_cells;
        self.generation += 1;
    }
}


#[macroquad::main("Lifegame")]
async fn main() {
    let mut game = Game::new(
        (screen_width() / SQUARE_SIZE) as usize,
        ((screen_height() - HEADER_HEIGHT) / SQUARE_SIZE) as usize,
    );
    game.randomize_cells();

    let mut run = false;

    loop {
        clear_background(LIGHTGRAY);

        if is_key_released(KeyCode::S) { run = !run; }
        if run {
            game.update();
        }
        if !run && is_key_released(KeyCode::N) { game.update(); }
        if is_key_released(KeyCode::R) { game.randomize_cells(); }
        if is_key_released(KeyCode::C) { game.clear_cells(); }
        if is_key_down(KeyCode::Left) { game.move_user(-1, 0); }
        if is_key_down(KeyCode::Right) { game.move_user(1, 0); }
        if is_key_down(KeyCode::Up) { game.move_user(0, -1); }
        if is_key_down(KeyCode::Down) { game.move_user(0, 1); }
        if is_key_released(KeyCode::Space) { game.reverse_by_user(); }

        game.draw(Point {x: 0.0, y: HEADER_HEIGHT});

        // let (mouse_x, mouse_y) = mouse_position();
        // draw_text(&format!("x: {}, y: {}", mouse_x, mouse_y), 5.0, screen_height() - 15.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
