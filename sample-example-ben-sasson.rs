use polynomial_commitment::{Commitment, PolynomialCommitment};
use rand::{thread_rng, Rng};

// Define the statement that you want to prove.
struct Statement {
    // The inputs needed to construct and verify the proof.
    inputs: Vec<u8>,
    // The polynomial that the prover and verifier will use to construct and verify the proof.
    polynomial: Vec<u8>,
}

// Define the proof.
struct Proof {
    commitment: Commitment,
    challenge: u64,
    response: u64,
}

impl Proof {
    fn new<R: Rng>(rng: &mut R, statement: &Statement) -> Self {
        // Generate the random values needed for the proof.
        let challenge: u64 = rng.gen();

        // Use the polynomial-commitment library to construct a commitment to the polynomial specified in the statement.
        let polynomial_commitment = PolynomialCommitment::new(statement.polynomial.as_slice());
        let commitment = polynomial_commitment.commit(challenge);

        // Compute the value of the response using the polynomial and the challenge.
        let response = polynomial_commitment.evaluate(challenge);

        // Construct the proof using the commitment, challenge, and response.
        Proof { commitment, challenge, response }
    }

    fn verify(&self, statement: & Statement) -> bool {
        // Use the polynomial-commitment library to verify the proof using the inputs and the polynomial specified in the statement.
        let polynomial_commitment = PolynomialCommitment::new(statement.polynomial.as_slice());
        polynomial_commitment.verify(&self.commitment, self.challenge, self.response)
    }
}
