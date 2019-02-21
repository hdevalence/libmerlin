//! A Rust testing harness for the C implementation.
// This entire crate is only for tests, so it's all unused without cfg(test)
#![allow(dead_code)]

#[cfg(test)]
extern crate keccak;
#[cfg(test)]
extern crate merlin;
#[cfg(test)]
extern crate rand;

// should be the right size?
#[repr(C)]
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
    fn randomized_transcript_conformance() {
        let num_runs = 100;
        for _ in 0..num_runs {
            random_transcript_run();
        }
    }

    #[test]
    fn basic_transcript_conformance() {
        let mut rs_transcript = merlin::Transcript::new(b"ConformanceTest");
        let mut c_transcript = Transcript::new(b"ConformanceTest");

        rs_transcript.commit_bytes(b"data", b"testdata");
        c_transcript.commit_bytes(b"data", b"testdata");

        let mut rs_chal = [0u8; 32];
        let mut c_chal = [0u8; 32];

        rs_transcript.challenge_bytes(b"chal", &mut rs_chal);
        c_transcript.challenge_bytes(b"chal", &mut c_chal);

        assert_eq!(c_chal, rs_chal);
    }

    fn random_transcript_run() {
        let mut rs_transcript = merlin::Transcript::new(b"ConformanceTest");
        let mut c_transcript = Transcript::new(b"ConformanceTest");

        let max_test_data_size = 64 * 1024;
        let mut message_data = vec![0u8; max_test_data_size];
        let mut rs_chal_buf = vec![0u8; max_test_data_size];
        let mut c_chal_buf = vec![0u8; max_test_data_size];

        let num_operations = 1_000;
        let mut rng = rand::thread_rng();

        for _ in 0..num_operations {
            let op_len = rng.gen_range(0, max_test_data_size);
            if rng.gen::<bool>() {
                rng.fill(&mut message_data[0..op_len]);
                rs_transcript.commit_bytes(b"data", &message_data[0..op_len]);
                c_transcript.commit_bytes(b"data", &message_data[0..op_len]);
            } else {
                rs_transcript.challenge_bytes(b"chal", &mut rs_chal_buf[0..op_len]);
                c_transcript.challenge_bytes(b"chal", &mut c_chal_buf[0..op_len]);
                assert_eq!(&rs_chal_buf[0..op_len], &c_chal_buf[0..op_len]);
            }
        }

        let mut rs_chal = [0u8; 32];
        let mut c_chal = [0u8; 32];

        rs_transcript.challenge_bytes(b"finalchal", &mut rs_chal);
        c_transcript.challenge_bytes(b"finalchal", &mut c_chal);

        assert_eq!(c_chal, rs_chal);
    }
}
