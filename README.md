# 🧬 Conway's Game of Life

การจำลองเซลลูลาร์ออโตมาตาคลาสสิกที่เขียนด้วย Rust สำหรับ Terminal

![Game of Life Demo](https://img.shields.io/badge/Language-Rust-orange.svg)
![No Dependencies](https://img.shields.io/badge/Dependencies-None-green.svg)
![Platform](https://img.shields.io/badge/Platform-Cross--Platform-blue.svg)

## 📖 เกี่ยวกับ Conway's Game of Life

Conway's Game of Life เป็นเกมจำลองชีวิตที่สร้างโดย John Horton Conway ในปี 1970 เป็นตัวอย่างของ **Cellular Automaton** ที่แสดงให้เห็นว่ากฎง่ายๆ สามารถสร้างพฤติกรรมที่ซับซ้อนและสวยงามได้

### กฎของเกม (The Rules)
1. **เซลล์ที่มีชีวิต** มีเพื่อนบ้าน < 2 ตัว → **ตาย** (เหงา)
2. **เซลล์ที่มีชีวิต** มีเพื่อนบ้าน 2-3 ตัว → **อยู่รอด**
3. **เซลล์ที่มีชีวิต** มีเพื่อนบ้าน > 3 ตัว → **ตาย** (แออัด)
4. **เซลล์ที่ตาย** มีเพื่อนบ้าน 3 ตัว → **เกิดใหม่**

## 🚀 การติดตั้งและรัน

### ความต้องการ
- Rust 1.70+ (ไม่ต้อง external dependencies!)

### วิธีรัน
```bash
# Clone หรือ download โค้ด
git clone <repo>
cd game-of-life

# รันโปรแกรム
cargo run

# หรือ compile แล้วรัน
cargo build --release
./target/release/game-of-life
```

## 🎮 วิธีใช้งาน

### เมื่อเริ่มโปรแกรม
```
🧬 Conway's Game of Life 🧬
Choose a pattern:
1. Glider
2. Blinker  
3. Block
4. Beacon
5. Random
6. Multiple patterns
```

**พิมพ์ตัวเลข 1-6 แล้วกด Enter**

### การควบคุม
- **เลือกรูปแบบ**: พิมพ์ตัวเลข 1-6
- **หยุดโปรแกรม**: `Ctrl+C`
- **ดูการเปลี่ยนแปลง**: รอดู (อัตโนมัติ)

## 🎨 รูปแบบที่มีให้เลือก

### 1. 🛸 Glider
```
 ██
   ██
██████
```
เคลื่อนที่ทแยงมุมไปเรื่อยๆ

### 2. ⚡ Blinker
```
██████  →  ██  →  ██████
           ██
           ██
```
กระพริบระหว่างแนวนอนและแนวตั้ง

### 3. ⬜ Block
```
████
████
```
อยู่นิ่ง ไม่เปลี่ยนแปลง

### 4. 💡 Beacon
```
████    
████  
    ████
    ████
```
กระพริบที่มุม

### 5. 🌟 Random
การกระจายเซลล์แบบสุ่ม (30% density)

### 6. 🎪 Multiple Patterns
รวมหลายรูปแบบในหน้าจอเดียว

## 🔧 การปรับแต่ง

### เปลี่ยนขนาดกริด
```rust
// แก้ไขใน main.rs บรรทัดที่สร้าง GameOfLife
let mut game = GameOfLife::new(60, 30); // กว้าง x สูง
```

### ปรับความเร็วในการแสดงผล
```rust
// หาบรรทัดที่มี thread::sleep ใน main.rs
thread::sleep(Duration::from_millis(100)); // เร็วขึ้น
thread::sleep(Duration::from_millis(500)); // ช้าลง
```

### สร้างรูปแบบใหม่
```rust
// เพิ่ม method ใหม่ใน impl GameOfLife
fn add_custom_pattern(&mut self, start_x: i32, start_y: i32) {
    // เพิ่มเซลล์ตามที่ต้องการ
    self.add_cell(start_x, start_y);
    self.add_cell(start_x + 1, start_y);
    // ...
}
```

### คอมไฟล์หลังแก้ไข
```bash
rustc -O main.rs  # คอมไฟล์ใหม่ทุกครั้งที่แก้โค้ด
```

## 💡 คุณสมบัติเด่น

- ✅ **ไม่ต้อง dependencies** - ใช้ Rust standard library เท่านั้น
- ✅ **ไฟล์เดียวจบ** - แค่ main.rs ไม่ต้องจัดการโปรเจค
- ✅ **คอมไฟล์ง่าย** - แค่ `rustc main.rs` เสร็จเลย
- ✅ **ประสิทธิภาพสูง** - ใช้ HashSet เก็บเฉพาะเซลล์ที่มีชีวิต
- ✅ **แสดงผลสวยงาม** - ใช้ Unicode blocks
- ✅ **รูปแบบหลากหลาย** - มี preset patterns พร้อมใช้
- ✅ **Cross-platform** - รันได้ทุก OS ที่รองรับ Rust

## 🧪 สิ่งที่น่าทดลอง

1. **ดู Glider ชน Block** - เลือก Multiple patterns
2. **Random pattern** - ดูว่าจะเกิดรูปแบบอะไรขึ้น
3. **แก้ไขขนาดกริด** - ทำให้ใหญ่ขึ้นดูผลลัพธ์
4. **ปรับความเร็ว** - ทำให้เร็วหรือช้าตามชอบ

## 📚 เรียนรู้เพิ่มเติม

- [Conway's Game of Life - Wikipedia](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
- [LifeWiki - Pattern Collection](https://conwaylife.com/wiki/Main_Page)
- [Cellular Automaton](https://en.wikipedia.org/wiki/Cellular_automaton)

## 🏗️ โครงสร้างโค้ด

```
main.rs              # ไฟล์เดียวจบ!
├── Position struct  # พิกัดเซลล์
├── GameOfLife struct# logic หลัก
├── Pattern methods  # รูปแบบต่างๆ
├── Display logic    # การแสดงผล
└── main()          # ฟังก์ชันหลัก
```

### ความเรียบง่าย
- **ไฟล์เดียว** - ไม่ต้องจัดการหลายไฟล์
- **ไม่ต้อง Cargo.toml** - แค่ rustc ก็พอ
- **Self-contained** - ทุกอย่างอยู่ใน main.rs

## 🤝 การมีส่วนร่วม

อยากปรับปรุงหรือเพิ่มคุณสมบัติ?
- เพิ่มรูปแบบใหม่ๆ
- ปรับปรุงการแสดงผล
- เพิ่มการควบคุมแบบ interactive
- สร้างระบบบันทึก/โหลด patterns