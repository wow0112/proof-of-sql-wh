use crate::base::proof::transcript_protocol::*;

use curve25519_dalek::scalar::Scalar;
use merlin::Transcript;

#[test]
fn test_challenge_scalars() {
    let zero = Scalar::from(0u64);
    let mut transcript = Transcript::new(b"multiplicationtest");
    let mut v: [Scalar; 3] = [zero; 3];
    transcript.challenge_scalars(&mut v, MessageLabel::SumcheckChallenge);
    assert_ne!(v[0], zero);
    assert_ne!(v[1], zero);
    assert_ne!(v[2], zero);
    assert_ne!(v[0], v[1]);
    assert_ne!(v[1], v[2]);
}