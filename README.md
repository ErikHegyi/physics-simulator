# Physics Simulator

## Goal
The goal of this program is to be able to simulate all of our current knowledge of physics.

## Method
The program is written in **Rust** and **Python**.

### Rust
The physical calculations are performed in Rust.
This includes things like vector algebra, gravitational force calculations, ...

### Python
Since Rust does not have inheritance, the Rust structs are exported to Python.\
Python also contains the graphics of the simulation, which were written using **PyOpenGL**.
