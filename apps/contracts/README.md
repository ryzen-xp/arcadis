# Arcadis Project

## Voting Contract 📥

The `voting-contract` located under `contracts/voting-contract` implements a **decentralized game voting system**.

```text
.
├── contracts
│   └── voting-contract
│       ├── src
│       │   ├── errors.rs   // error types for panic and validation control
│       │   ├── events.rs   // event types for emitting blockchain events
│       │   ├── lib.rs      // main entry point that connects modules and exposes the contract
│       │   ├── test.rs     // unit tests verifying contract behavior
│       │   ├── traits.rs   // trait defining the external interface of the Voting contract.
│       │   └── voting.rs   // core business logic for the voting system
│       └── Cargo.toml
├── Cargo.toml
└── README.md

```

This contract allows users on the Arcadis platform to:

- **Register new games** by providing a custom ID and name.
- **Vote (like)** games while ensuring each user can only vote once per game.
- **Track the total votes** and retrieve game metadata like name, ID, and creator.
- **Prevent duplicate voting** by associating each user to their vote history.

The storage is organized efficiently to separate:

- Game metadata
- User voting history
- Total games counter

It is designed to be **simple**, **modular**, and **extensible** for future enhancements like downvotes, time-limited voting, or additional game metadata.

---

## 🚀 Getting Started

To build and test any contract:

```bash
make build
make test
```

or manually:

```bash
cd contracts/voting-contract
cargo build
cargo test
```

---

## 📦 Notes on Structure

- All Soroban smart contracts are placed under the `contracts/` directory, each in their own subfolder.
- Each contract (like `voting-contract`) has:
  - Its own `Cargo.toml` for defining dependencies and metadata.
  - A `src/` directory containing the `lib.rs` (main contract code) and `test.rs` (unit tests).
- The top-level `Cargo.toml` defines a **workspace** that includes all contracts.
- Additional frontend apps, SDKs, or CLI tools can also be added at the top level if needed.

> **Note:** If you initialized the project with templates or examples using flags like `--with-example` or `--frontend-template`, extra folders will be added automatically under `contracts/` or alongside it.
