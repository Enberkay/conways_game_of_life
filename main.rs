use std::collections::HashSet;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    fn neighbors(&self) -> Vec<Position> {
        vec![
            Position::new(self.x - 1, self.y - 1),
            Position::new(self.x - 1, self.y),
            Position::new(self.x - 1, self.y + 1),
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
            Position::new(self.x + 1, self.y - 1),
            Position::new(self.x + 1, self.y),
            Position::new(self.x + 1, self.y + 1),
        ]
    }
}

struct GameOfLife {
    live_cells: HashSet<Position>,
    cell_ages: std::collections::HashMap<Position, u32>, // อายุของเซลล์
    width: i32,
    height: i32,
    generation: u32,
}

impl GameOfLife {
    fn new(width: i32, height: i32) -> Self {
        GameOfLife {
            live_cells: HashSet::new(),
            cell_ages: std::collections::HashMap::new(),
            width,
            height,
            generation: 0,
        }
    }

    fn add_cell(&mut self, x: i32, y: i32) {
        let pos = Position::new(x, y);
        self.live_cells.insert(pos);
        self.cell_ages.insert(pos, 0); // เซลล์ใหม่อายุ 0
    }

    fn is_alive(&self, pos: &Position) -> bool {
        self.live_cells.contains(pos)
    }

    fn count_live_neighbors(&self, pos: &Position) -> usize {
        pos.neighbors()
            .iter()
            .filter(|neighbor| self.is_alive(neighbor))
            .count()
    }

    fn next_generation(&mut self) {
        let mut candidates = HashSet::new();

        // เพิ่มเซลล์ที่มีชีวิตและเพื่อนบ้านทั้งหมด
        for cell in &self.live_cells {
            candidates.insert(*cell);
            for neighbor in cell.neighbors() {
                candidates.insert(neighbor);
            }
        }

        let mut next_generation = HashSet::new();
        let mut next_ages = std::collections::HashMap::new();

        for candidate in candidates {
            let live_neighbors = self.count_live_neighbors(&candidate);
            let is_alive = self.is_alive(&candidate);

            // กฎของ Conway's Game of Life
            if is_alive && (live_neighbors == 2 || live_neighbors == 3) {
                // เซลล์ที่มีชีวิตและมีเพื่อนบ้าน 2-3 ตัว จะอยู่รอด
                next_generation.insert(candidate);
                let current_age = self.cell_ages.get(&candidate).unwrap_or(&0);
                next_ages.insert(candidate, current_age + 1); // เพิ่มอายุ
            } else if !is_alive && live_neighbors == 3 {
                // เซลล์ที่ตายและมีเพื่อนบ้าน 3 ตัว จะเกิดใหม่
                next_generation.insert(candidate);
                next_ages.insert(candidate, 0); // เซลล์ใหม่อายุ 0
            }
        }

        self.live_cells = next_generation;
        self.cell_ages = next_ages;
        self.generation += 1;
    }

    fn display(&self) {
        // ล้างหน้าจอ
        print!("\x1B[2J\x1B[1;1H");

        // แสดงหัวข้อสีสวย
        print!("\x1B[1;36m🧬 Conway's Game of Life 🧬\x1B[0m\n");
        print!(
            "\x1B[1;33mGrid: {}x{} | Generation: {} | Living cells: {}\x1B[0m\n",
            self.width,
            self.height,
            self.generation,
            self.live_cells.len()
        );

        // สถิติอายุเซลล์
        if !self.cell_ages.is_empty() {
            let max_age = self.cell_ages.values().max().unwrap_or(&0);
            let avg_age = self.cell_ages.values().sum::<u32>() as f32 / self.cell_ages.len() as f32;
            print!(
                "\x1B[1;35mMax age: {} | Average age: {:.1}\x1B[0m\n",
                max_age, avg_age
            );
        }
        println!();

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x, y);
                if self.is_alive(&pos) {
                    let age = self.cell_ages.get(&pos).unwrap_or(&0);
                    let color_code = match age {
                        0 => "\x1B[1;92m",      // เขียวสด - เซลล์ใหม่
                        1..=2 => "\x1B[1;96m",  // ฟ้าสด - เซลล์หนุ่ม
                        3..=5 => "\x1B[1;94m",  // น้ำเงิน - เซลล์วัยกลางคน
                        6..=10 => "\x1B[1;95m", // ม่วง - เซลล์แก่
                        _ => "\x1B[1;91m",      // แดง - เซลล์แก่มาก
                    };
                    print!("{}██\x1B[0m", color_code);
                } else {
                    print!("\x1B[90m░░\x1B[0m"); // เทาอ่อน - พื้นหลัง
                }
            }
            println!();
        }

        // คำแนะนำ
        print!("\x1B[2;37mPress Ctrl+C to stop\x1B[0m\n");
        io::stdout().flush().unwrap();
    }

    // สร้างรูปแบบ Glider
    fn add_glider(&mut self, start_x: i32, start_y: i32) {
        let glider_pattern = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];

        for (dx, dy) in glider_pattern {
            self.add_cell(start_x + dx, start_y + dy);
        }
    }

    // สร้างรูปแบบ Blinker
    fn add_blinker(&mut self, start_x: i32, start_y: i32) {
        for i in 0..3 {
            self.add_cell(start_x + i, start_y);
        }
    }

    // สร้างรูปแบบ Block (ไม่เปลี่ยนแปลง)
    fn add_block(&mut self, start_x: i32, start_y: i32) {
        for x in 0..2 {
            for y in 0..2 {
                self.add_cell(start_x + x, start_y + y);
            }
        }
    }

    // สร้างรูปแบบ Beacon
    fn add_beacon(&mut self, start_x: i32, start_y: i32) {
        // บล็อกแรก
        self.add_cell(start_x, start_y);
        self.add_cell(start_x + 1, start_y);
        self.add_cell(start_x, start_y + 1);

        // บล็อกที่สอง
        self.add_cell(start_x + 2, start_y + 2);
        self.add_cell(start_x + 3, start_y + 2);
        self.add_cell(start_x + 3, start_y + 3);
    }

    // สุ่มเซลล์
    fn randomize(&mut self, density: f32) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now().hash(&mut hasher);
        let mut seed = hasher.finish();

        for y in 0..self.height {
            for x in 0..self.width {
                // Linear Congruential Generator สำหรับสุ่ม
                seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                let random = (seed >> 16) as f32 / 65536.0;

                if random < density {
                    self.add_cell(x, y);
                }
            }
        }
    }
}

fn main() {
    println!("🧬 Conway's Game of Life 🧬");
    println!("Choose a pattern:");
    println!("1. Glider");
    println!("2. Blinker");
    println!("3. Block");
    println!("4. Beacon");
    println!("5. Random (sparse)");
    println!("6. Random (dense)");
    println!("7. Multiple patterns");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    let mut game = GameOfLife::new(40, 20);

    match choice {
        "1" => game.add_glider(5, 5),
        "2" => game.add_blinker(19, 10),
        "3" => game.add_block(19, 9),
        "4" => game.add_beacon(18, 8),
        "5" => game.randomize(0.15), // ลดความหนาแน่นลง
        "6" => {
            game.add_glider(2, 2);
            game.add_blinker(35, 15);
            game.add_block(35, 2);
            game.add_beacon(2, 15);
            game.add_glider(20, 8);
        }
        _ => game.add_glider(5, 5),
    }

    println!("\nStarting simulation... Press Ctrl+C to stop");
    thread::sleep(Duration::from_secs(2));

    // รันจำลอง
    for _generation in 0..1000 {
        game.display();

        thread::sleep(Duration::from_millis(300)); // ช้าลงหน่อยเพื่อชมสี
        game.next_generation();

        // หยุดถ้าไม่มีเซลล์เหลือ
        if game.live_cells.is_empty() {
            println!("\n\x1B[1;31m💀 All cells died! Simulation ended.\x1B[0m");
            break;
        }
    }
}
