[![Rust](https://github.com/Rikatemu/Vindium-Server/actions/workflows/rust.yml/badge.svg?branch=main&event=push)](https://github.com/Rikatemu/Vindium-Server/actions/workflows/rust.yml)
# Vindium Server

![Vindium](https://raw.githubusercontent.com/Rikatemu/Vindium-Unity/main/readme_img.gif)

A very basic game server written in [Rust](https://www.rust-lang.org/) utilizing [Tokio](https://tokio.rs/) and a [Unity client-side integration](https://github.com/Rikatemu/Vindium-Unity).

# Features
- [x] Client-authoritative movement
- [x] Client-side movement interpolation
- [ ] Entity state sync
- [ ] Child transform sync
- [ ] Authentication
- [ ] Spatial hashing interest management
- [ ] Map interest management
- [ ] Teleport/speed hack checker (No server-authoritative movement planned)
- [ ] AI behaviour controller
- [ ] AI pathfinding
- [ ] Control plane (one game world processed by multiple server instances to handle large amounts of entities)
