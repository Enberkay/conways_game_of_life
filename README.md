# Conway's Game of Life

This project is an implementation of Conway's Game of Life using Rust and Macroquad.

Conway's Game of Life, created by John Horton Conway in 1970, is a classic example of a cellular automaton. It demonstrates how simple rules governing individual cells can lead to complex and sometimes unexpected emergent behaviors.

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
A pattern where cells are initially placed randomly with a given density. This setup is useful for observing emergent behaviors and studying how chaos and order evolve in the system.

### 3. Block  
A stable 2x2 square of live cells that does not change over time. It represents a simple still life pattern, useful as a fixed structure or memory in more complex configurations.

### 4. Blinker  
A line of three live cells that oscillates between horizontal and vertical orientation every generation. It is the simplest oscillator, demonstrating periodic behavior in the system.

### 5. Beacon  
Consists of two adjacent 2x2 blocks that flash alternately. This oscillator shows how multiple stable groups can interact to create rhythmic behavior.

---

## Why These Patterns Matter

Conway's Game of Life is known to be Turing complete, which means it can simulate any computation given the right configuration of cells. The patterns above serve as fundamental components:

- **Gliders** can be thought of as signals transmitting information.  
- **Still life patterns** like blocks serve as stable memory units.  
- **Oscillators** like blinkers and beacons function as clocks or timing devices.

Understanding these simple patterns provides insight into how complex systems and computation can emerge from simple rules, highlighting the importance of cellular automata in fields like mathematics, computer science, and artificial life.

---

## Controls

- Arrow keys (↑ / ↓) to navigate menus  
- Enter to confirm selection  
- Escape to go back or cancel

---

## Features

- Selectable screen resolution  
- Multiple simulation patterns  
- Bounded simulation area  
- Generation and FPS display  

---

## Future Plans

- Pause and resume functionality  
- Mouse interaction to toggle cells  
- Pattern import and export  
- Wrap-around (toroidal) board mode  

---