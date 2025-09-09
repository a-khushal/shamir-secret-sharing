## Shamir Secret Sharing

Minimal implementation of Shamir's Secret Sharing over the secp256k1 prime.

Requirements: Node.js 18+ (for `globalThis.crypto`).

Build and run (inside `v1/ts`):
```bash
tsc -b && node dist/index.js
```

example: Outputs 5 shares for a sample secret with threshold 3.

Rust (inside `v1/rs`):
```bash
cargo test -- --nocapture
```

License: MIT.

