# getrandom-runtime-seeded
A custom getrandom implementation that uses a ChaChaRng instance set at runtime. 

### Setup
To register the function, we first depend on getrandom-runtime-seeded and getrandom with the custom feature in Cargo.toml:

```toml
[dependencies]
getrandom-runtime-seeded = "0.1"
getrandom = { version = "0.2", features = ["custom"] }
``

Then, we register the function in src/lb.rs:

use getrandom-runtime-seeded::always_fail;
use getrandom::register_custom_getrandom;

register_custom_getrandom!(always_fail);