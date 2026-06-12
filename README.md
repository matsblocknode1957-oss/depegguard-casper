# DepegGuard — Signal Logger (Casper)

On-chain depeg signal registry for the DepegGuard Strategy suite. Records HEDGE/EXIT signals emitted by the PegCheck oracle into persistent Casper smart contract storage, providing an immutable audit trail for automated stablecoin risk management.

Part of the [DepegGuard Strategy Suite](../depegguard-skill).

---

## Deployed Contract

| Field | Value |
|---|---|
| Network | Casper Testnet |
| Package Hash | `fa9029503df4d1e156e25a613cf261439d8cc9a13e11093270b80643d1fa92c9` |
| Deploy TX | [View on CSPR.live](https://testnet.cspr.live/transaction/e88676ba1baa5b44b3c5070f4a17dfd797adfbb5234059ac13bc404d801082ac) |
| RPC Endpoint | `https://node.testnet.casper.network/rpc` |

---

## Architecture

```
PegCheck Oracle
      │
      │  price deviation data (USDT, USDC, DAI, FRAX, LUSD, DOLA, PYUSD)
      ▼
x402 Wrapper (depegguard-x402)
  Express · /api/signal
      │
      │  POST log_signal  (coin, signal, deviation_bps, timestamp)
      ▼
SignalLogger Contract  ◄──── get_last_n / get_count (read-only queries)
  Casper Testnet
```

### Components

**`signal-logger/`** — Casper smart contract (this repo)
- Written with the [Odra](https://odra.dev) framework v2.7.2
- Stores `SignalEntry` records: coin name, signal type, deviation in basis points, unix timestamp
- Exposes three entry points: `log_signal`, `get_count`, `get_last_n`

**`depegguard-x402/`** — x402-authenticated API wrapper (companion repo)
- Express server exposing `POST /api/signal`
- Queries PegCheck for live peg deviation across USDT, USDC, DAI, FRAX, LUSD, DOLA, PYUSD
- Translates depeg events into on-chain `log_signal` calls via the Casper JSON-RPC

**`depegguard-skill/`** — Strategy execution layer (companion repo)
- Consumes logged signals to drive automated hedge/exit strategies

### Signal Schema

| Field | Type | Description |
|---|---|---|
| `coin` | `String` | Stablecoin ticker (e.g. `"USDC"`) |
| `signal` | `String` | `"HEDGE"` or `"EXIT"` |
| `deviation_bps` | `u64` | Price deviation from $1.00 peg in basis points (150 = −1.5%) |
| `timestamp` | `u64` | Unix timestamp in seconds (caller-supplied) |
| `index` | `u64` | Auto-incremented log index |

---

## Contract Entry Points

### `log_signal(coin, signal, deviation_bps, timestamp)`
Records a new depeg signal. Increments the internal counter and writes a `SignalEntry` at the next index slot.

### `get_count() → u64`
Returns the total number of signals logged to date.

### `get_last_n(n) → Vec<SignalEntry>`
Returns up to `n` most recent signals, ordered newest-first.

---

## Deploy Instructions

### Prerequisites

- Rust with the `nightly-2026-01-01` toolchain and `wasm32-unknown-unknown` target
- [Odra CLI](https://odra.dev/docs/cli): `cargo install odra-cli`
- A funded Casper Testnet account and its PEM secret key

### Build

```bash
cd signal-logger
cargo odra build
```

### Deploy to Testnet

Set the required environment variables:

```bash
export CASPER_NODE_ADDRESS=https://node.testnet.casper.network/rpc
export CASPER_CHAIN_NAME=casper-test
export CASPER_SECRET_KEY=/path/to/secret_key.pem
```

Then deploy:

```bash
cargo odra deploy -b casper-livenet --bin deploy
```

The deployer prints the contract address on success. Verify on [CSPR.live](https://testnet.cspr.live).

### Run Tests (mock environment)

```bash
cargo test
```

---

## Toolchain

| Component | Version |
|---|---|
| Rust toolchain | `nightly-2026-01-01` |
| Wasm target | `wasm32-unknown-unknown` |
| Odra framework | `2.7.2` |
| Casper network | Testnet (`casper-test`) |
