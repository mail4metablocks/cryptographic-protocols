package main

import (
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/crypto/sha3"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/phoreproject/snark/bellman"
)

type MyCircuit struct {
	// Circuit inputs
	a *big.Int
	b *big.Int
	// Circuit output
	c *big.Int
}

func (c *MyCircuit) Witness(a, b *big.Int) (witness []*big.Int) {
	c.a = a
	c.b = b
	c.c = new(big.Int).Add(a, b)
	return []*big.Int{a, b, c.c}
}

func (c *MyCircuit) Constraints() (inputs, outputs [][]byte) {
	inputs = [][]byte{
		bellman.Int64ToWitness(c.a),
		bellman.Int64ToWitness(c.b),
	}
	outputs = [][]byte{
		bellman.Int64ToWitness(c.c),
	}
	return inputs, outputs
}

func main() {
	circuit := &MyCircuit{}
	witness := circuit.Witness(big.NewInt(1), big.NewInt(2))

	provingKey := &bellman.ProvingKey{
		Alpha:     make([]byte, 32),
		Beta:      make([]byte, 32),
		Delta:     make([]byte, 32),
		IC:        make([]*bellman.IC, 0),
		Polynomial: make([]*bellman.Poly, 0),
	}

	proof, _ := bellman.CreateProof(provingKey, circuit)
	fmt.Printf("Proof:\n%+v\n", proof)

	// Verify the proof
	inputs := make([][]byte, len(witness))
	for i, w := range witness {
		inputs[i] = bellman.Int64ToWitness(w)
	}

	hash := sha3.NewKeccak256()
	rlp.Encode(hash, inputs)
	inputsHash := hash.Sum(nil)

	if bellman.VerifyProof(provingKey, proof, inputsHash) {
		fmt.Println("Proof is valid!")
	} else {
		fmt.Println("Proof is invalid :(")
	}
}
