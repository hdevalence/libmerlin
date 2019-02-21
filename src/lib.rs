//! A Rust testing harness for the C implementation.
// This entire crate is only for tests, so it's all unused without cfg(test)
#![allow(unused_code)]

#[cfg(test)]
extern crate keccak;
#[cfg(test)]
extern crate merlin;
#[cfg(test)]
extern crate rand;

// should be the right size?
struct CTranscript {
    state: [u8; 203],
}

extern "C" {
    fn keccakf(state: *mut [u64; 25]);
    fn merlin_transcript_init(mctx: *mut CTranscript, label: *const u8, label_len: usize);
    fn merlin_transcript_commit_bytes(
        mctx: *mut CTranscript,
        label: *const u8,
        label_len: usize,
        data: *const u8,
        data_len: usize,
    );
    fn merlin_transcript_challenge_bytes(
        mctx: *mut CTranscript,
        label: *const u8,
        label_len: usize,
        buffer: *mut u8,
        buffer_len: usize,
    );
}

struct Transcript {
    mctx: CTranscript,
}

impl Transcript {
    pub fn new(label: &'static [u8]) -> Transcript {
        let mut mctx = CTranscript { state: [0u8; 203] };
        unsafe {
            merlin_transcript_init(&mut mctx, label.as_ptr(), label.len());
        }
        Transcript { mctx }
    }

    pub fn commit_bytes(&mut self, label: &'static [u8], message: &[u8]) {
        unsafe {
            merlin_transcript_commit_bytes(
                &mut self.mctx,
                label.as_ptr(),
                label.len(),
                message.as_ptr(),
                message.len(),
            );
        }
    }

    pub fn challenge_bytes(&mut self, label: &'static [u8], dest: &mut [u8]) {
        unsafe {
            merlin_transcript_challenge_bytes(
                &mut self.mctx,
                label.as_ptr(),
                label.len(),
                dest.as_mut_ptr(),
                dest.len(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn rust_keccak_vs_c_keccak() {
        let iterations = 1_000;
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

    #[test]
    fn basic_transcript_conformance() {
        let mut rs_transcript = merlin::Transcript::new(b"ConformanceTest");
        let mut c__transcript = Transcript::new(b"ConformanceTest");

        rs_transcript.commit_bytes(b"data", b"testdata");
        c__transcript.commit_bytes(b"data", b"testdata");

        let mut rs_chal = [0u8; 32];
        let mut c__chal = [0u8; 32];

        rs_transcript.challenge_bytes(b"chal", &mut rs_chal);
        c__transcript.challenge_bytes(b"chal", &mut c__chal);

        assert_eq!(c__chal, rs_chal);
    }
}
