//! A Rust testing harness for the C implementation.

#[cfg(test)]
extern crate keccak;
#[cfg(test)]
extern crate merlin;
#[cfg(test)]
extern crate rand;

#[cfg(test)]
extern "C" {
    fn keccakf(state: &mut [u64; 25]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn rust_keccak_vs_c_keccak() {
        let iterations = 1_000_000;
        let initial_state: [u64; 25] = rand::thread_rng().gen();

        let mut rust_state = initial_state;
        let mut c_state = initial_state;

        for _ in 0..iterations {
            keccak::f1600(&mut rust_state);
            unsafe {
                keccakf(&mut c_state);
            }
            assert_eq!(rust_state, c_state);
        }
    }
}
