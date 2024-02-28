# getrandom-runtime-seeded

A custom getrandom implementation that uses a ChaChaRng instance set at runtime.

### Setup

To register the custom getrandom function, we first depend on `getrandom-runtime-seeded` and `getrandom` with the custom
feature in
`Cargo.toml`:

```toml
[dependencies]
getrandom-runtime-seeded = "0.1"
getrandom = { version = "0.2", features = ["custom"] }
```

Then, we register the custom implementation in `src/lib.rs`:

```rust
use getrandom::register_custom_getrandom;
use getrandom_runtime_seeded::seeded_with_runtime_chacha_rng;

register_custom_getrandom!(seeded_with_runtime_chacha_rng);
```

Finally, in `src/main.rs` create and set a `ChaChaRng` instance by calling `init_getrandom` with a 32-byte seed:

```rust
use getrandom_runtime_seeded::init_getrandom;

// use a real seed here from a secure source (not a hardcoded one)
init_getrandom([0u8; 32]);
```

### Notes:
* This implementation is designed for single threaded use cases as it sets `ChaChaRng` in a `thread_local` context.
