use macroquad::prelude::*;
use macroquad::rand::gen_range;
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

        let color = Color::new(0.15, 0.15, 0.15, 1.0);
        for x in 0..=self.width {
            draw_line(
                (x * self.cell_size) as f32,
                0.0,
                (x * self.cell_size) as f32,
                (self.height * self.cell_size) as f32,
                1.0,
                color,
            );
        }
        for y in 0..=self.height {
            draw_line(
                0.0,
                (y * self.cell_size) as f32,
                (self.width * self.cell_size) as f32,
                (y * self.cell_size) as f32,
                1.0,
                color,
            );
        }

        draw_rectangle_lines(
            0.0,
            0.0,
            (self.width * self.cell_size) as f32,
            (self.height * self.cell_size) as f32,
            3.0,
            RED,
        );

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
                if gen_range(0.0, 1.0) < density {
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

async fn choose_resolution(sizes: &[(i32, i32)]) -> usize {
    let mut selected = 1;
    loop {
        clear_background(DARKGRAY);
        draw_text("Select screen size:", 20.0, 50.0, 30.0, WHITE);
        for (i, (w, h)) in sizes.iter().enumerate() {
            let marker = if i == selected { ">" } else { " " };
            draw_text(&format!("{} {}x{}", marker, w, h), 40.0, 100.0 + i as f32 * 30.0, 25.0, WHITE);
        }
        draw_text("Enter to confirm", 20.0, 250.0, 25.0, GREEN);
        if is_key_pressed(KeyCode::Up) {
            selected = (selected + sizes.len() - 1) % sizes.len();
        }
        if is_key_pressed(KeyCode::Down) {
            selected = (selected + 1) % sizes.len();
        }
        if is_key_pressed(KeyCode::Enter) {
            break;
        }
        next_frame().await;
    }
    next_frame().await;
    selected
}

async fn choose_pattern() -> Option<usize> {
    let pattern_names = ["Glider", "Random", "Block", "Blinker", "Beacon"];
    let mut selected = 0;
    loop {
        clear_background(DARKBLUE);
        draw_text("Select pattern:", 20.0, 50.0, 30.0, WHITE);
        for (i, name) in pattern_names.iter().enumerate() {
            let marker = if i == selected { ">" } else { " " };
            draw_text(&format!("{} {}", marker, name), 40.0, 100.0 + i as f32 * 30.0, 25.0, WHITE);
        }
        draw_text("Enter to start | Esc to go back", 20.0, 270.0, 25.0, GREEN);
        if is_key_pressed(KeyCode::Up) {
            selected = (selected + pattern_names.len() - 1) % pattern_names.len();
        }
        if is_key_pressed(KeyCode::Down) {
            selected = (selected + 1) % pattern_names.len();
        }
        if is_key_pressed(KeyCode::Enter) {
            break Some(selected);
        }
        if is_key_pressed(KeyCode::Escape) {
            break None;
        }
        next_frame().await;
    }
}

async fn run_simulation(width: i32, height: i32, pattern: usize) {
    let cell_size = 10;
    let grid_width = width / cell_size;
    let grid_height = height / cell_size;
    let mut game = GameOfLife::new(grid_width, grid_height, cell_size);

    match pattern {
        0 => game.add_glider(grid_width / 2, grid_height / 2),
        1 => game.add_random(0.2),
        2 => {
            for x in 10..12 {
                for y in 10..12 {
                    game.add_cell(x, y);
                }
            }
        }
        3 => {
            for i in 0..3 {
                game.add_cell(grid_width / 2 + i, grid_height / 2);
            }
        }
        4 => {
            game.add_cell(10, 10);
            game.add_cell(11, 10);
            game.add_cell(10, 11);
            game.add_cell(12, 13);
            game.add_cell(13, 12);
            game.add_cell(13, 13);
        }
        _ => {}
    }

    loop {
        game.next_generation();
        game.draw();
        next_frame().await;
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let sizes = [(640, 480), (800, 600), (1024, 768), (1920, 1080)];

    loop {
        let size_index = choose_resolution(&sizes).await;
        let (screen_width, screen_height) = sizes[size_index];

        if let Some(pattern_index) = choose_pattern().await {
            run_simulation(screen_width, screen_height, pattern_index).await;
            break;
        }

        // If pattern menu returned None, user pressed Esc → go back to resolution menu
    }
}
