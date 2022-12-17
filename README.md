# cryptographic-protocols


Zk-SNARKs (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge) and zk-STARKs (Zero-Knowledge Scalable Transparent Argument of Knowledge) are both cryptographic protocols that allow one party (the prover) to prove to another party (the verifier) that they know a certain piece of information, without revealing any information about the actual content of that information.

The key difference between the two protocols is that zk-SNARKs are based on elliptic curve cryptography, while zk-STARKs are based on polynomial commitments. This means that zk-SNARKs are generally faster and more efficient, but they also require a trusted setup phase in which the public parameters of the system are generated. This trusted setup can be a security concern, as it requires all parties involved to be present and to generate the parameters in a secure manner. In contrast, zk-STARKs do not require a trusted setup and are therefore considered to be more secure.

Both zk-SNARKs and zk-STARKs are used in a variety of applications, including anonymous transactions, confidential contracts, and voting systems. In all of these applications, the goal is to allow parties to prove that they have certain information without revealing the actual content of that information.

zkSNARK and zkSTARK are zero-knowledge proof systems that can be used to provide privacy and security in blockchain implementations. They allow a user (the prover) to prove to another party (the verifier) that they know the solution to a problem, without revealing the actual solution. This can be useful in a variety of applications, including blockchain transactions where the parties want to keep the details of the transaction private.

Groth '16 is a zero-knowledge proof system that was introduced in the paper "On the Size of Pairing-Based Non-Interactive Arguments" by Jens Groth. It is based on the use of pairings, which are a type of mathematical function that can be used to link elements in different mathematical groups. Groth '16 uses a specific type of pairing called a "bilinear pairing," which allows for efficient proof construction and verification.

![image](https://user-images.githubusercontent.com/117555665/208240870-9a32b167-c201-4951-8a6f-b626af2aabc5.png)


Ben-Sasson '18 is a zero-knowledge proof system that was introduced in the paper "Scalable, transparent, and post-quantum secure computational integrity" by Eli Ben-Sasson and others. It is based on the use of polynomial commitments, which are a type of cryptographic commitment scheme that allows for efficient proof construction and verification.

![image](https://user-images.githubusercontent.com/117555665/208240887-ca9fd178-ca13-483a-ba7c-8e793f3e8d2a.png)


PLONK is a zero-knowledge proof system that was introduced in the paper "PLONK: A Practical and Trustworthy Implementation of Fully Homomorphic Encryption" by Eli Ben-Sasson and others. It is based on the use of multivariate polynomials, which are a type of mathematical function that can be used to represent and manipulate data. PLONK uses specific types of multivariate polynomials called "inner product polynomials," which allow for efficient proof construction and verification.

![image](https://user-images.githubusercontent.com/117555665/208240914-9fe950b2-1bef-4e6b-89da-a316ab696914.png)



These proof systems can be used in blockchain implementations to provide privacy and security for transactions, by allowing parties to prove the correctness of their transactions without revealing the details of the transactions. They can also be used to verify the integrity of data stored on the blockchain, by allowing parties to prove that the data has not been tampered with without revealing the actual contents of the data.

All of these proof systems have been widely studied and have been used in various applications, including in the development of blockchain and cryptocurrency technologies. They have also inspired the development of other proof systems, such as zk-Rollup and zk-Sync, which aim to improve upon the efficiency and scalability of these systems.


zk-SNARKs protocol:

![image](https://user-images.githubusercontent.com/117555665/208236100-45220589-e547-4888-b013-f1274047f5f1.png)


zk-STARKs protocol:

![image](https://user-images.githubusercontent.com/117555665/208236120-3b4d7597-49d4-4e04-affa-afbbf281c772.png)


### Some libraries that you can use to implement these proof systems in Rust are:

pairing: a Rust library for working with bilinear pairings, which can be used to implement Groth '16.
polynomial-commitment: a Rust library for working with polynomial commitments, which can be used to implement Ben-Sasson '18.
inner-product-polynomials: a Rust library for working with inner product polynomials, which can be used to implement PLONK.

## Groth '16:

Define the statement that you want to prove, and the set of inputs that the prover and verifier will need in order to construct and verify the proof.

Use a library such as pairing to implement the bilinear pairing operations needed for the proof.

Use a library such as rand to generate random values needed for the proof.

Implement the prover's algorithm, which constructs a proof of the statement using the inputs and the bilinear pairing operations.

Implement the verifier's algorithm, which verifies the proof using the inputs and the bilinear pairing operations.

## Ben-Sasson '18:

Define the statement that you want to prove, and the set of inputs that the prover and verifier will need in order to construct and verify the proof.

Use a library such as polynomial-commitment to implement the polynomial commitment operations needed for the proof.

Use a library such as rand to generate random values needed for the proof.

Implement the prover's algorithm, which constructs a proof of the statement using the inputs and the polynomial commitment operations.

Implement the verifier's algorithm, which verifies the proof using the inputs and the polynomial commitment operations.

## PLONK:

Define the statement that you want to prove, and the set of inputs that the prover and verifier will need in order to construct and verify the proof.

Use a library such as inner-product-polynomials to implement the inner product polynomial operations needed for the proof.

Use a library such as rand to generate random values needed for the proof.

Implement the prover's algorithm, which constructs a proof of the statement using the inputs and the inner product polynomial operations.

Implement the verifier's algorithm, which verifies the proof using the inputs and the inner product polynomial operations.


