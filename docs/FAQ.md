# FAQ

Things to know when using Wasm bindings as a newbie dev...

## What are Wasm bindings?

Wasm stands for WebAssembly Modules. It's like a translator between CosmWasm smart contracts [hosted on Kiichain] and host Go-based chain modules.

Since smart contracts are sandboxed, they can't mint tokens, handle special types of queries, etc...
Wasmbindings was made exactly to bridge that gap.

## So, what does it actually do?

It can perform the following features:

- Custom messages sent by contracts like 'Send a transaction to the EVM module'...
- Custom queries from contracts to the blockchain like 'What’s the current price of BTC from the oracle?'
- And many more!

## How is this different from CosmWasm?

CosmWasm is the base engine for running contracts.
wasmbindings is the set of extra tools that expand what those contracts can do on Kiichain.

Without wasmbindings, contracts are limited to standard CosmWasm features.
With it, they can do chain-specific actions (e.g. minting tokens via TokenFactory or accessing Oracle price data).

Eg: You can use a custom message like ``KiichainMsg::TokenFactoryMint{...}``

## How do you call a function?

You can use structured CosmosMsg or WasmQuery types.

Example,

```rust
let msg = CosmosMsg::Custom(
    KiichainMsg::TokenFactoryMint {
        denom: "mycoin".to_string(),
        amount: Uint128::new(1000),
        recipient: info.sender.to_string(),
    }
);
```

## Can I call any function from wasm?

Nope, my friend. You can call only the ones that the chain explicitly exposes via its bindings, not just any function.

You can call:

- CosmWasm's Built-in functions
- Custom functions via `wasmbindings`

