# 1. What are WebAssembly (Wasm) bindings?

#### Wasm bindings expose custom blockchain functionality (like Kiichain-specific messages or queries) to smart contracts written in Rust. These act as bridges between the blockchain runtime and the contract logic.

---
# 2. What is `CosmWasm` and how does this project extend it?

#### CosmWasm is a smart contract platform for Cosmos-based blockchains. This project extends **CosmWasm** by adding Kiichain-specific bindings (via `KiichainMsg` and `KiichainQuery`) to enable features unique to Kiichain, such as token factory operations.
---
# 3. Can I call any blockchain function from Wasm?

#### No. You can only call functions that are explicitly exposed to smart contracts through bindings like `KiichainMsg` and `KiichainQuery`. These bindings are defined in the Rust code and must be supported by the blockchain's runtime.
---
# 4. What is `KiichainMsg`?
#### `KiichainMsg` is a Rust enum that defines **custom executable messages** (like minting tokens or changing admin) that your contract can send to interact with Kiichain-specific features.
---
# 5. What is `KiichainQuery`?

#### `KiichainQuery` defines custom queries you can make to the blockchain, such as fetching admin information for a token denom or checking contract state related to Kiichain extensions.
---
# 6. What’s the purpose of the reflect contract?
#### Found in `contracts/reflect`, the reflect contract is an example that demonstrates how to:
* Send messages using `KiichainMsg`
* Query using `KiichainQuery`
* Proxy messages
* Handle submessage replies
* Manage contract ownership

It’s a great starting point for beginners.
---
# 7. How do I build the project locally?
#### Install Rust and run:
```bash
cargo build
```
#### For optimized release builds:
```bash
./scripts/build_release.sh
```
---
# 8. How do I generate schema files for frontend use?
#### Run the schema export example:
```bash
cargo run --example schema
```
#### This outputs JSON schemas used by frontends or integrators to understand contract messages.
---
# 9. How do I include these bindings in my own contract?
#### In your Cargo.toml:
```bash
[dependencies]
kiichain-bindings = { path = "../packages/bindings" }
```
#### And in your code:
```bash
use token_bindings::{KiichainMsg, KiichainQuery};
```
---
# 10. What toolchain do I need to compile CosmWasm smart contracts?
#### Install the wasm target:
```bash
rustup target add wasm32-unknown-unknown
```
#### Ensure cargo-generate is installed:
```bash
cargo install cargo-generate
```
#### You also need a local chain (e.g., [Kiichain](https://github.com/KiiChain/kiichain)) or [wasmd](https://github.com/CosmWasm/wasmd).
---
# 11. How is token minting done via contracts?
#### Using KiichainMsg::TokenFactory::MintTokens, you can mint tokens with a message like:
```json
{
  "custom": {
    "token_factory": {
      "mint_tokens": {
        "denom": "example",
        "amount": "1000",
        "mint_to_address": "address"
      }
    }
  }
}
```
This is demonstrated in the reflect contract's execute section.
---
# 12. How do I query the token factory admin?
#### You can use a query like:
```json
{
  "chain": {
    "request": {
      "custom": {
        "token_factory": {
          "admin": {
            "denom": "factory/example"
          }
        }
      }
    }
  }
}
```
---
# 13. Can I simulate messages before sending them?
#### Yes — you can use test environments like cw-multi-test in Rust to simulate contract execution before deploying on-chain.
---
# 14. How do I debug a failing Wasm smart contract?
#### Use the RUST_BACKTRACE=1 flag when running tests:
```bash
RUST_BACKTRACE=1 cargo test
```
#### You can also inspect logs from the local Kiichain node. For more, see this StackOverflow thread.
---
# 15. Is this code compatible with CosmWasm standard tools?
#### Yes. The bindings are designed to be CosmWasm-compatible, meaning you can use tools like:
* `wasmd`
* CosmJS
* `cw-multi-test`
* Contract schema generators

However, the messages and queries specific to Kiichain only work on Kiichain or compatible chains that support these bindings.
---
### Happy Coding