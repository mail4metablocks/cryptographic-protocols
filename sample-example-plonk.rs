use inner_product_polynomials::{
    Commitment, InnerProductProof, InnerProductWitness, Polynomial, RandomOracle,
    ScalarChallenge,
};
use rand::{thread_rng, Rng};

// Define the statement that you want to prove.
struct Statement {
    // The inputs needed to construct and verify the proof.
    inputs: Vec<u8>,
    // The polynomials that the prover and verifier will use to construct and verify the proof.
    polynomials: Vec<Polynomial>,
}

// Define the proof.
struct Proof {
    commitments: Vec<Commitment>,
    challenge: ScalarChallenge,
    responses: Vec<u64>,
}

impl Proof {
    fn new<R: Rng>(rng: &mut R, statement: &Statement) -> Self {
        // Generate the random values needed for the proof.
        let random_oracle = RandomOracle::new(rng);

        // Use the inner-product-polynomials library to construct a witness for the polynomials specified in the statement.
        let witness = InnerProductWitness::new(statement.polynomials.as_slice(), &random_oracle);

        // Use the inner-product-polynomials library to construct commitments to the polynomials.
        let commitments = witness.commitments();

        // Use the inner-product-polynomials library to generate a challenge based on the inputs and the commitments.
        let challenge = ScalarChallenge::new(
            statement.inputs.as_slice(),
            &commitments,
            &random_oracle,
        );

        // Use the inner-product-polynomials library to compute the responses using the witness and the challenge.
        let responses = witness.responses(&challenge);

        // Construct the proof using the commitments, challenge, and responses.
        Proof { commitments, challenge, responses }
    }

    fn verify(&self, statement: & Statement) -> bool {
        // Use the inner-product-polynomials library to verify the proof using the inputs and the polynomials specified in the statement.
        InnerProductProof::verify(
            statement.inputs.as_slice(),
            statement.polynomials.as_slice(),
            &self.commitments,
            &self.challenge,
            &self.responses,
        )
    }
}
