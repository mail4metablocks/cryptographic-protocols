package main

import (
	"fmt"

	"github.com/phoreproject/starling/gadgets"
)

type EvenOddCircuit struct {
	// Circuit input
	x gadgets.UInt64
	// Circuit output
	isEven gadgets.Bool
}

func (c *EvenOddCircuit) Define(curveID gadgets.CurveID) error {
	// Define the input variable
	c.x.Assign(curveID, "x")

	// Define the output variable
	c.isEven.Assign(curveID, "is_even")

	// Enforce the constraint (x % 2) = is_even
	c.x.Mod(c.isEven.Not(), 2)
	c.x.ConstrainEqual(c.isEven)

	return nil
}

func main() {
	circuit := &EvenOddCircuit{}
	if err := circuit.Define(gadgets.Bn256); err != nil {
		fmt.Println(err)
		return
	}

	// Generate a proof
	prover := gadgets.NewProver(circuit)
	inputs := []*gadgets.Field{gadgets.NewUInt64(5)}
	if err := prover.GenerateProof(inputs); err != nil {
		fmt.Println(err)
		return
	}

	// Verify the proof
	verifier := gadgets.NewVerifier(circuit)
	if err := verifier.VerifyProof(prover.Proof, inputs); err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println("Proof is valid!")
}
