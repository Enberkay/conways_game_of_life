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
            if pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height {
                continue; // Skip out-of-bound cells
            }

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

        // Draw live cells
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

        // Draw grid lines
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

        // Draw red border around simulation area
        draw_rectangle_lines(
            0.0,
            0.0,
            (self.width * self.cell_size) as f32,
            (self.height * self.cell_size) as f32,
            3.0,
            RED,
        );

        // Draw generation and FPS text
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

    fn add_block(&mut self, x: i32, y: i32) {
        for dx in 0..2 {
            for dy in 0..2 {
                self.add_cell(x + dx, y + dy);
            }
        }
    }

    fn add_blinker(&mut self, x: i32, y: i32) {
        for dx in 0..3 {
            self.add_cell(x + dx, y);
        }
    }

    fn add_beacon(&mut self, x: i32, y: i32) {
        self.add_cell(x, y);
        self.add_cell(x + 1, y);
        self.add_cell(x, y + 1);
        self.add_cell(x + 2, y + 3);
        self.add_cell(x + 3, y + 2);
        self.add_cell(x + 3, y + 3);
    }

    fn add_rpentomino(&mut self, x: i32, y: i32) {
        let pattern = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];
        for (dx, dy) in pattern {
            self.add_cell(x + dx, y + dy);
        }
    }

    fn add_acorn(&mut self, x: i32, y: i32) {
        let pattern = [(1, 0), (3, 1), (0, 2), (1, 2), (4, 2), (5, 2), (6, 2)];
        for (dx, dy) in pattern {
            self.add_cell(x + dx, y + dy);
        }
    }

    fn add_diehard(&mut self, x: i32, y: i32) {
        let pattern = [(6, 0), (0, 1), (1, 1), (1, 2), (5, 2), (6, 2), (7, 2)];
        for (dx, dy) in pattern {
            self.add_cell(x + dx, y + dy);
        }
    }

    fn add_gosper_gun(&mut self, x: i32, y: i32) {
        let pattern = [
            (24, 0),
            (22, 1),
            (24, 1),
            (12, 2),
            (13, 2),
            (20, 2),
            (21, 2),
            (34, 2),
            (35, 2),
            (11, 3),
            (15, 3),
            (20, 3),
            (21, 3),
            (34, 3),
            (35, 3),
            (0, 4),
            (1, 4),
            (10, 4),
            (16, 4),
            (20, 4),
            (21, 4),
            (0, 5),
            (1, 5),
            (10, 5),
            (14, 5),
            (16, 5),
            (17, 5),
            (22, 5),
            (24, 5),
            (10, 6),
            (16, 6),
            (24, 6),
            (11, 7),
            (15, 7),
            (12, 8),
            (13, 8),
        ];
        for (dx, dy) in pattern {
            self.add_cell(x + dx, y + dy);
        }
    }

    fn add_pentadecathlon(&mut self, x: i32, y: i32) {
        let pattern = [
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (1, -1),
            (1, 1),
            (4, -1),
            (4, 1),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
        ];
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
            draw_text(
                &format!("{} {}x{}", marker, w, h),
                40.0,
                100.0 + i as f32 * 30.0,
                25.0,
                WHITE,
            );
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
    let pattern_names = [
        "Glider",
        "Random",
        "Block",
        "Blinker",
        "Beacon",
        "R-pentomino",
        "Acorn",
        "Diehard",
        "Gosper Gun",
        "Pentadecathlon",
    ];
    let mut selected = 0;
    loop {
        clear_background(DARKBLUE);
        draw_text("Select pattern:", 20.0, 50.0, 30.0, WHITE);
        for (i, name) in pattern_names.iter().enumerate() {
            let marker = if i == selected { ">" } else { " " };
            draw_text(
                &format!("{} {}", marker, name),
                40.0,
                100.0 + i as f32 * 30.0,
                25.0,
                WHITE,
            );
        }
        draw_text("Enter to start | Esc to go back", 20.0, 400.0, 25.0, GREEN);

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
        2 => game.add_block(10, 10),
        3 => game.add_blinker(grid_width / 2, grid_height / 2),
        4 => game.add_beacon(10, 10),
        5 => game.add_rpentomino(grid_width / 2, grid_height / 2),
        6 => game.add_acorn(10, 10),
        7 => game.add_diehard(10, 10),
        8 => game.add_gosper_gun(1, 1),
        9 => game.add_pentadecathlon(grid_width / 2 - 4, grid_height / 2),
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

        // If user pressed Esc on pattern menu, go back to resolution menu
    }
}
