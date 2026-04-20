# 📝 Todo List — Soroban Smart Contract

A decentralized To-Do List application built on the **Stellar blockchain** using **Soroban** smart contracts. This project was developed as part of the Stellar Web3 Workshop to demonstrate how on-chain data storage and smart contract logic work together in a practical use case.

---

## 📖 Description

This smart contract allows users to manage their to-do tasks entirely on-chain. Every task (todo) is stored directly on the Stellar blockchain via Soroban's instance storage, ensuring data is transparent, tamper-proof, and not controlled by any central entity.

The contract is built with [`soroban-sdk`](https://docs.rs/soroban-sdk) using Rust and is designed to be deployed and tested in **Soroban Studio** or via the Stellar CLI.

---

## ✨ Features

| Feature | Function | Description |
|---|---|---|
| ✅ Create Todo | `create_todo` | Add a new task with a title and description. Status defaults to **Pending**. |
| 📋 Get All Todos | `get_todos` | Retrieve the full list of todos stored on-chain. |
| ✏️ Update Todo | `update_todo` | Edit the title and description of an existing todo by its ID. |
| ☑️ Complete Todo | `complete_todo` | Mark a specific todo as **Completed** by its ID. |
| 🗑️ Delete Todo | `delete_todo` | Permanently remove a todo from storage by its ID. |
| 🧹 Clear Completed | `clear_completed` | Bulk-remove all todos with a **Completed** status at once. |

---

## 🏗️ Data Structure

### `Todo`
```rust
pub struct Todo {
    id: u64,           // Unique identifier (randomly generated)
    title: String,     // Short title of the task
    description: String, // Detailed description of the task
    status: TodoStatus, // Current status of the task
}
```

### `TodoStatus`
```rust
pub enum TodoStatus {
    Pending,    // Task is not yet done
    Completed,  // Task has been finished
}
```

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (with `wasm32-unknown-unknown` target)
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/stellar-cli)
- [Soroban Studio](https://studio.soroban.io/) *(recommended for quick testing)*

### Install Rust WASM target

```bash
rustup target add wasm32-unknown-unknown
```

### Build the Contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled `.wasm` file will be located at:
```
target/wasm32-unknown-unknown/release/todo_soroban.wasm
```

### Run Tests

```bash
cargo test
```

---

## 🧪 Testing in Soroban Studio

1. Open [Soroban Studio](https://studio.soroban.io/)
2. Paste the smart contract code from `src/lib.rs`
3. Click **Build** to compile the contract
4. Use the **Invoke** panel to call each function:

| Step | Function | Parameters |
|---|---|---|
| 1 | `create_todo` | `title: "Buy groceries"`, `description: "Milk and eggs"` |
| 2 | `get_todos` | *(none)* — returns the full list |
| 3 | `complete_todo` | `id: <copy ID from get_todos result>` |
| 4 | `update_todo` | `id`, `title: "New title"`, `description: "New desc"` |
| 5 | `delete_todo` | `id: <copy ID from get_todos result>` |
| 6 | `clear_completed` | *(none)* — removes all completed todos |

---

## 📁 Project Structure

```
todo-soroban/
├── Cargo.toml       # Project dependencies and build config
└── src/
    ├── lib.rs       # Main smart contract logic
    └── test.rs      # Unit tests for all contract functions
```

---

## 🌐 About the Tech Stack

| Technology | Role |
|---|---|
| **Stellar** | Layer-1 blockchain network |
| **Soroban** | Smart contract platform built on Stellar |
| **Rust** | Programming language for writing the contract |
| **soroban-sdk** | SDK for building and testing Soroban contracts |

---

## 📚 Resources

- [Stellar Developer Docs](https://developers.stellar.org/)
- [Soroban Documentation](https://soroban.stellar.org/)
- [Soroban Studio](https://studio.soroban.io/)
- [soroban-sdk on crates.io](https://crates.io/crates/soroban-sdk)

---

## 👨‍💻 Author
ID CONTRACT : CAW6B5HAP7CPHLC372NNKEB6C7UIVBRYMOF2GTABUTJLSWZYNY7VAQEI
Built with ❤️ during the **Stellar Web3 Workshop** — an introduction to blockchain, Stellar, and Soroban smart contract development.