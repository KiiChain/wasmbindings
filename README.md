> This repository is based on [Token Bindings](https://github.com/CosmosContracts/token-bindings)

# Kiichain CosmWasm Bindings

This repository provides **CosmWasm smart contract bindings for the Kiichain blockchain**, exposing custom messages and queries.

## 🧪 Example: Reflect Contract

Located in `contracts/reflect`, this smart contract demonstrates how to:

- Send `CosmosMsg<KiichainMsg>` and `SubMsg<KiichainMsg>`.
- Use `QueryRequest<KiichainQuery>` for custom chain queries.
- Proxy messages and handle submessage replies.
- Change and query contract ownership.

### Instantiate

```json
{}
```

### Execute

```json
{
  "reflect_msg": {
    "msgs": [
      {
        "custom": {
          "token_factory": {
            "mint_tokens": {
              "denom": "...",
              "amount": "...",
              "mint_to_address": "..."
            }
          }
        }
      }
    ]
  }
}
```

### Query

```json
{
  "chain": {
    "request": {
      "custom": {
        "token_factory": {
          "admin": {
            "denom": "factory/..."
          }
        }
      }
    }
  }
}
```

---

## 🔧 Getting Started

### Requirements

- Rust (with `wasm32-unknown-unknown` target)
- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
- [`wasmd`](https://github.com/CosmWasm/wasmd) or compatible chain like [Kiichain](https://github.com/KiiChain/kiichain)

### Build and Test

```bash
# Install dependencies
cargo build

# Run schema export for bindings
cargo run --example schema

# Build compressed
./scripts/build_release.sh
```

---

## 📦 Crate Usage

To use the Kiichain bindings in your contract:

```toml
# In your Cargo.toml
[dependencies]
kiichain-bindings = { path = "../packages/bindings" }
```

And in your contract code:

```rust
use token_bindings::{KiichainMsg, KiichainQuery};
```
---
📘 New to this repo? Check out our [FAQ](docs/FAQ.md) for common questions!
