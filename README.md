# sarb

A Statistical Arbitrage Bot. It holds opposing positions in two correlated crypto assets and enters a trade when their spread stretches past a statistically significant threshold — then waits for it to snap back.

## How it works

- **OLS regression** computes the hedge ratio (β) between each pair, giving the exact sizing ratio for a dollar-neutral position.
- **Ornstein-Uhlenbeck fit** estimates the half-life of mean reversion. Pairs above 360 minutes are treated as non-stationary and disabled.
- **Z-score signals** normalise the spread against its rolling mean and standard deviation. Entries fire at |Z| > 2.0, exits at |Z| ≤ 0.5.
- **Pair states** (`Accumulating`, `Active`, `Unwinding`, `Dead`) are recomputed on every tick from current conditions — not a sequential pipeline.
- **WebSocket feed** connects to Binance aggTrade streams for all symbols on one multiplexed connection, with automatic ping/pong and 5-second reconnect backoff.
- **60-second metronome** drives all statistical work via `tokio::select!`, consuming a batch of cached live prices each cycle.
- **Persistence** appends each pair's prices to a per-pair CSV every tick and writes open positions to `data/active_positions.json` on every trade event. On restart, both are read back — price history reconstructs the rolling windows, positions restore open trades.
- **Zero-beta guard** suppresses signals when the hedge ratio collapses near zero, blocking directional trades disguised as pair trades.

## Universe

15 pairs across 3 co-integration sectors (all verified above $10M daily volume on Binance as of March 22, 2026):

| Sector | Assets | Pairs |
|---|---|---|
| L1 | SOL, AVAX, ADA, SUI | 6 |
| AI | FET, WLD, TAO | 3 |
| DeFi / Infra | LINK, FIL, ETHFI, RDNT | 6 |

## Configuration

`CAPACITY` is a named constant in `main.rs`. The other three are literals in `universe.rs`.

| Parameter | Dev | Prod | Description |
|---|---|---|---|
| `CAPACITY` | `5` | `2880` | Rolling window size (2880 = 48h at 60s intervals) |
| `entry_z` | `2.0` | `2.0` | Z-score threshold to open a position |
| `exit_z` | `0.5` | `0.5` | Z-score threshold to close a position |
| `max_half_life` | `360.0` | `360.0` | Max mean reversion half-life in minutes |

## Run

```bash
cargo run --release
```
