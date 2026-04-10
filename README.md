# CLI Rust RPG AdventuRust

A CLI-based role-playing game built with Rust as a learning experience project.

## Motivation

Following an interview for a Rust-focused role, I received valuable feedback regarding the importance of demonstrating core language fundamentals through hands-on implementation. I built AdventuRust as a direct response to solidify my understanding of Rust's ownership system, match patterns, and CLI state management while building a scalable foundation for a portfolio-ready project.

## Description

A text-based RPG where players battle through encounters, manage inventory, and collect gold. Features:
- Random enemy encounters (Goblin, Slime, Orc, Lesser Demon, Skeleton)
- Shop system to buy potions
- Rest between battles to heal and prepare
- Progressive difficulty scaling
- Inventory management
- Defend mechanic (halves damage for 2 turns)

## How to Play

1. Clone the repository
2. Run with `cargo run`
3. Choose actions: Attack, Defend, or Use Potion
4. Collect gold from defeated enemies
5. Visit the shop to buy supplies
6. Progress through 11 levels to face the boss

## Game States

- Encounter - Combat with random enemies
- Rest - Heal and prepare for next battle
- Shop - Buy potions with gold
- Boss - Final battle (coming soon)

## Lessons Learned

Through building AdventuRust, I gained hands-on experience with:
- Rust Ownership - Managing player, enemy, and inventory state without garbage collection
- Pattern Matching - State machine implementation using enums and match expressions
- CLI I/O - Handling user input and formatted output
- Project Structure - Organizing code across multiple modules

## Upcoming Features

- Boss battle with unique attack patterns
- More enemy types
- Additional items and upgrades

## Built With

- Rust
- No external dependencies (pure Rust standard library)

## License

MIT
