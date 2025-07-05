# Conway's Game of Life

This project is an implementation of Conway's Game of Life using Rust and Macroquad.

Conway's Game of Life, created by John Horton Conway in 1970, is a classic example of a cellular automaton. It demonstrates how simple rules governing individual cells can lead to complex and sometimes unexpected emergent behaviors.

Conway's Game of Life ถูกคิดค้นโดย John Horton Conway นักคณิตศาสตร์ชาวอังกฤษในปี 1970 ครับ

เขาสร้างเกมนี้ขึ้นมาเพื่อเป็นตัวอย่างของ cellular automaton ซึ่งแสดงให้เห็นว่ากฎง่าย ๆ บางอย่างสามารถก่อให้เกิดพฤติกรรมที่ซับซ้อนและน่าสนใจได้

Game of Life จึงเป็นหนึ่งในโมเดลสำคัญที่ช่วยให้นักวิทยาศาสตร์และนักคณิตศาสตร์เข้าใจเรื่อง emergence และ complex systems มากขึ้น

---

## The Rules

1. Any live cell with fewer than two live neighbors dies (underpopulation).  
2. Any live cell with two or three live neighbors survives to the next generation.  
3. Any live cell with more than three live neighbors dies (overpopulation).  
4. Any dead cell with exactly three live neighbors becomes a live cell (reproduction).

---

## Simulation Patterns Explained

### 1. Glider  
A small pattern of five live cells that moves diagonally across the board over time. The glider is a fundamental "spaceship" in the Game of Life, capable of transporting information and used as a building block in larger complex structures.

### 2. Random  
Cells placed randomly with a specified density to observe emergent chaotic and ordered behaviors.

### 3. Block  
A stable 2x2 square of live cells that remains unchanged, representing a simple still life.

### 4. Blinker  
A line of three live cells that oscillates between horizontal and vertical every generation, the simplest oscillator.

### 5. Beacon  
Two adjacent 2x2 blocks that flash alternately, an oscillator showing rhythmic interaction.

### 6. R-pentomino  
A small initial pattern of five cells that evolves for a surprisingly long time before stabilizing, showing complex growth.

### 7. Acorn  
A sparse pattern of seven cells that evolves over thousands of generations into a large chaotic field, demonstrating long-term growth.

### 8. Diehard  
A pattern of seven cells that lasts for 130 generations before disappearing, illustrating transient complexity.

### 9. Gosper Glider Gun  
A famous pattern that periodically emits gliders indefinitely, the first known "gun" producing moving patterns.

### 10. Pentadecathlon  
An oscillator with a period of 15 generations, one of the longest-period simple oscillators.

---

## Why These Patterns Matter

Conway's Game of Life is known to be Turing complete, meaning it can simulate any computation given the right initial conditions. The patterns above serve as fundamental components:

- **Gliders** act as signals transmitting information across the grid.  
- **Still life patterns** like blocks act as stable memory or static elements.  
- **Oscillators** such as blinkers, beacons, and pentadecathlons serve as timing devices or clocks.  
- **Spaceships and guns** (gliders, Gosper Glider Gun) enable construction of logic gates and complex machines.  
- **Long-lived patterns** (R-pentomino, Acorn, Diehard) show how simple starts can generate complex and rich behavior.

Understanding these patterns helps us grasp how complexity and computation emerge from simple local rules, influencing fields like mathematics, computer science, physics, and artificial life.

---

## Controls

- Arrow keys (↑ / ↓) to navigate menus  
- Enter to confirm selection  
- Escape to go back or cancel

---
