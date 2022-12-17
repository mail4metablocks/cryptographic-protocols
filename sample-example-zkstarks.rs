extern crate starling;

use starling::{
    Circuit,
    ConstraintSystem,
    SynthesisError,
};

struct EvenOddCircuit {
    // Circuit input
    x: Option<u64>,
    // Circuit output
    is_even: Option<bool>,
}

impl Circuit<starling::bn256::Bn256> for EvenOddCircuit {
    fn synthesize<CS: ConstraintSystem<starling::bn256::Bn256>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError>
    {
        // Allocate the input variable
        let x = cs.alloc(|| "x", || self.x.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocate the output variable
        let is_even = cs.alloc(|| "is_even", || self.is_even.ok_or(SynthesisError::AssignmentMissing))?;

        // Enforce the constraint (x % 2) = is_even
        cs.enforce(
            || "x % 2 = is_even",
            |lc| lc + x,
            |lc| lc + CS::one(),
            |lc| lc + is_even,
        );

        Ok(())
    }
}
