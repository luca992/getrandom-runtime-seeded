extern crate alloc;

use std::cell::RefCell;

use getrandom::Error;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

thread_local!(static CHACHA_RNG: RefCell<Option<ChaChaRng>> = RefCell::new(None));

pub fn init_getrandom(rng_seed: [u8; 32]) {
    CHACHA_RNG.with(|x| {
        *x.borrow_mut() = Some(ChaChaRng::from_seed(rng_seed));
    });
}

pub fn seeded_with_env_block_random(buf: &mut [u8]) -> Result<(), Error> {
    CHACHA_RNG.with(|rng| {
        match rng.borrow_mut().as_mut() {
            Some(rng) => {
                // Generate random data
                rng.fill_bytes(buf);
                Ok(())
            }
            None => Err(Error::UNEXPECTED),
        }
    })
}
