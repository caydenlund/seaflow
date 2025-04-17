# SeaFlow

A modular sea of nodes IR and compiler toolkit written in Rust.

## Overview

SeaFlow is designed as a collection of independent but interoperable crates that implement modern compiler techniques with a focus on:

- **Sea of nodes intermediate representation**
- **Extensible optimization pipelines**
- **Target-agnostic code generation**
- **Educational clarity** (with detailed docs and examples)

## Current Status

Features are planned but not implemented!

## Planned Features

### Core Infrastructure

- [ ] **Basic IR Components**
  - [ ] Control flow and data flow nodes
  - [ ] SSA-form representation
  - [ ] Dominator tree analysis
  - [ ] Loop detection and analysis

- [ ] **Optimization Framework**
  - [ ] Modular pass infrastructure
  - [ ] Common subexpression elimination
  - [ ] Dead code elimination
  - [ ] Loop-invariant code motion
  - [ ] Inlining heuristics

### Frontend Support

- [ ] **Language Support**
  - [ ] Simple C-like language parser
  - [ ] ML-style functional language frontend
  - [ ] Source mapping and debugging info

### Backend Targets

- [ ] **Code Generation**
  - [ ] x86-64 backend
  - [ ] ARM64 backend
  - [ ] WASM backend
  - [ ] Custom VM bytecode

### Tooling

- [ ] **Developer Tools**
  - [ ] IR visualizer (Graphviz/DOT output)
  - [ ] Compiler debug REPL
  - [ ] Benchmarking harness
  - [ ] Fuzzing infrastructure

### Advanced Features

- [ ] **Parallel Compilation**
  - [ ] Multi-threaded optimization passes
  - [ ] Parallel code generation

- [ ] **Profile-Guided Optimization**
  - [ ] Instrumentation support
  - [ ] Profile data reader

## License

Dual-licensed under either of the following, at your option:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
