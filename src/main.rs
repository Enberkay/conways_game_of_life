use macroquad::prelude::*;
use macroquad::rand::gen_range; // ✅ ใช้ random จาก macroquad
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

struct GameOfLife {
    live_cells: HashSet<Position>,
    width: i32,
    height: i32,
    cell_size: i32,
    generation: u32,
}

impl GameOfLife {
    fn new(width: i32, height: i32, cell_size: i32) -> Self {
        Self {
            live_cells: HashSet::new(),
            width,
            height,
            cell_size,
            generation: 0,
        }
    }

    fn add_cell(&mut self, x: i32, y: i32) {
        self.live_cells.insert(Position(x, y));
    }

    fn is_alive(&self, pos: &Position) -> bool {
        self.live_cells.contains(pos)
    }

    fn count_neighbors(&self, pos: &Position) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor = Position(pos.0 + dx, pos.1 + dy);
                if self.is_alive(&neighbor) {
                    count += 1;
                }
            }
        }
        count
    }

    fn next_generation(&mut self) {
        let mut new_set = HashSet::new();
        let mut candidates = HashSet::new();

        for cell in &self.live_cells {
            candidates.insert(*cell);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    candidates.insert(Position(cell.0 + dx, cell.1 + dy));
                }
            }
        }

        for pos in candidates {
            let neighbors = self.count_neighbors(&pos);
            if self.is_alive(&pos) && (neighbors == 2 || neighbors == 3) {
                new_set.insert(pos);
            } else if !self.is_alive(&pos) && neighbors == 3 {
                new_set.insert(pos);
            }
        }

        self.live_cells = new_set;
        self.generation += 1;
    }

    fn draw(&self) {
        clear_background(BLACK);

        for cell in &self.live_cells {
            let x = cell.0 * self.cell_size;
            let y = cell.1 * self.cell_size;
            draw_rectangle(
                x as f32,
                y as f32,
                self.cell_size as f32,
                self.cell_size as f32,
                GREEN,
            );
        }

        draw_text(
            &format!("Generation: {} | FPS: {:.1}", self.generation, get_fps()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
    }

    fn add_random(&mut self, density: f32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let r = gen_range(0.0, 1.0); // ✅ ใช้ gen_range ของ macroquad
                if r < density {
                    self.add_cell(x, y);
                }
            }
        }
    }

    fn add_glider(&mut self, x: i32, y: i32) {
        let pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for (dx, dy) in pattern {
            self.add_cell(x + dx, y + dy);
        }
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut selected_size = 1;
    let sizes = [(640, 480), (800, 600), (1024, 768)];
    let mut pattern = 0; // 0 = glider, 1 = random

    loop {
        clear_background(DARKGRAY);

        draw_text("Select screen size:", 20.0, 50.0, 30.0, WHITE);
        for (i, (w, h)) in sizes.iter().enumerate() {
            let marker = if i == selected_size { ">" } else { " " };
            draw_text(&format!("{} {}x{}", marker, w, h), 40.0, 90.0 + i as f32 * 30.0, 25.0, WHITE);
        }

        draw_text("1 = Glider | 2 = Random", 20.0, 200.0, 25.0, WHITE);
        draw_text("Press Enter to start", 20.0, 240.0, 25.0, GREEN);

        if is_key_pressed(KeyCode::Up) {
            selected_size = (selected_size + sizes.len() - 1) % sizes.len();
        }
        if is_key_pressed(KeyCode::Down) {
            selected_size = (selected_size + 1) % sizes.len();
        }
        if is_key_pressed(KeyCode::Key1) {
            pattern = 0;
        }
        if is_key_pressed(KeyCode::Key2) {
            pattern = 1;
        }
        if is_key_pressed(KeyCode::Enter) {
            break;
        }

        next_frame().await;
    }

    let (screen_width, screen_height) = sizes[selected_size];
    let cell_size = 10;
    let width = screen_width / cell_size;
    let height = screen_height / cell_size;

    let mut game = GameOfLife::new(width, height, cell_size);

    match pattern {
        0 => game.add_glider(width / 2, height / 2),
        1 => game.add_random(0.2),
        _ => {}
    }

    loop {
        game.next_generation();
        game.draw();
        next_frame().await;
    }
}
