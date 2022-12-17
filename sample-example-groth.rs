use pairing::{CurveAffine, Engine, Field, PairingCurveAffine};
use rand::{thread_rng, Rng};

// Define the statement that you want to prove.
struct Statement {
    // The inputs needed to construct and verify the proof.
    inputs: Vec<u8>,
    // The computation that the prover and verifier will use to construct and verify the proof.
    computation: fn(Vec<u8>) -> bool,
}

// Define the proof.
struct Proof {
    a: PairingCurveAffine<Engine>,
    b: PairingCurveAffine<Engine>,
    c: PairingCurveAffine<Engine>,
}

impl Proof {
    fn new<R: Rng>(rng: &mut R, statement: &Statement) -> Self {
        // Generate the random values needed for the proof.
        let a: PairingCurveAffine<Engine> = rng.gen();
        let b: PairingCurveAffine<Engine> = rng.gen();

        // Compute the value of c using the computation specified in the statement.
        let c = (statement.computation)(statement.inputs);

        // Construct the proof using the values of a, b, and c.
        Proof { a, b, c }
    }

    fn verify(&self, statement: &Statement) -> bool {
        // Use the inputs and the computation specified in the statement to verify the proof.
        let lhs = Engine::pairing(self.a, self.b);
        let rhs = Engine::pairing(self.c, Engine::g1().generator());
        lhs == rhs
    }
}
