# 🚀 Basic Token Creator (Soroban Smart Contract)

## 📌 Project Description

Basic Token Creator is a Soroban smart contract built on the Stellar blockchain. It enables users to create and manage a simple fungible token with essential features like initialization, minting, transferring, and balance tracking.

This project is designed as a beginner-friendly implementation to understand how token systems work in Soroban.

---

## ⚙️ What It Does

* Creates a token with a name and symbol
* Assigns an admin (contract owner)
* Allows minting of new tokens
* Enables token transfers between users
* Tracks balances on-chain

---

## ✨ Features

* 🪙 Token initialization (name & symbol)
* 🔐 Admin-controlled minting
* 🔁 Token transfer functionality
* 📊 Balance checking
* ⚡ Lightweight and efficient contract
* 🧱 Built using Soroban SDK (Rust)

---

## 🧱 Contract Functions

| Function                          | Description                         |
| --------------------------------- | ----------------------------------- |
| `initialize(admin, name, symbol)` | Initializes the token               |
| `mint(to, amount)`                | Mints tokens to a user (admin only) |
| `transfer(from, to, amount)`      | Transfers tokens between users      |
| `balance(user)`                   | Returns the balance of a user       |

---

## 🛠 Build Instructions

```bash
stellar contract build
```

---

## 🚀 Deploy Command

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/basic_token.wasm --source alice --network testnet --alias basic_token
```

---

## 🔗 Deployed Smart Contract Link

```
XXX
```

*(Replace with your actual contract ID or Stellar Explorer link)*

---

## 📂 Project Structure

```
.
├── src/
│   └── lib.rs        # Smart contract logic
├── Cargo.toml        # Rust dependencies
└── target/           # Build output (WASM file)
```

---

## 🧪 Example Usage

### Initialize Token

```bash
stellar contract invoke --id basic_token --source alice --network testnet -- initialize --admin alice --name TOKEN --symbol TKN
```

### Mint Tokens

```bash
stellar contract invoke --id basic_token --source alice --network testnet -- mint --to alice --amount 1000
```

### Transfer Tokens

```bash
stellar contract invoke --id basic_token --source alice --network testnet -- transfer --from alice --to bob --amount 100
```

### Check Balance

```bash
stellar contract invoke --id basic_token --network testnet -- balance --user alice
```

---

## 🧠 Tech Stack

* Rust 🦀
* Soroban SDK
* Stellar Testnet

---

## 📚 Future Improvements

* Add total supply tracking
* Add burn functionality
* Add allowance/approve system (ERC20 style)
* Add frontend UI (React + Stellar wallet)

---

## 🤝 Contributing

Contributions are welcome! Feel free to fork and improve the project.

---

## 📜 License

MIT License
# basic-token-creator
