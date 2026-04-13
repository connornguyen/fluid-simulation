# Fluid Simulation

A 2D fluid simulation built with Rust and Bevy. The goal is to simulate two fluids interacting — like pouring milk into a coffee cup (top-down view).

## Tech Stack

- **Language:** Rust
- **Engine:** [Bevy](https://bevyengine.org/) `0.18.1`

## Prerequisites

### 1. Install Rust
https://rust-lang.org/tools/install/

### 2. Install Bevy
https://bevy.org/learn/quick-start/getting-started/setup/

## Getting Started

```bash
git clone https://github.com/connornguyen/fluid-simulation.git
cd fluid-simulation
cargo run
```

> First build may take a few minutes as Bevy and its dependencies compile.

## Controls

| Input | Action |
|-------|--------|
| Left click + hold still | Pour milk (circle grows) |
| Left click + drag | Draw milk trail (circle shrinks while moving) |

