extern crate bellman;
extern crate pairing;

use bellman::{
    Circuit,
    ConstraintSystem,
    SynthesisError,
};
use pairing::{
    Engine,
    Field,
};

struct MyCircuit<E: Engine> {
    // Circuit inputs
    a: Option<E::Fr>,
    b: Option<E::Fr>,
    // Circuit output
    c: Option<E::Fr>,
}

impl<E: Engine> Circuit<E> for MyCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError>
    {
        // Allocate the first input variable
        let a = cs.alloc(|| "a", || self.a.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocate the second input variable
        let b = cs.alloc(|| "b", || self.b.ok_or(SynthesisError::AssignmentMissing))?;

        // Allocate the output variable
        let c = cs.alloc(|| "c", || self.c.ok_or(SynthesisError::AssignmentMissing))?;

        // Enforce the constraint a + b = c
        cs.enforce(
            || "a + b = c",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + c,
        );

        Ok(())
    }
}
