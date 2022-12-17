# cryptographic-protocols


Zk-SNARKs (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge) and zk-STARKs (Zero-Knowledge Scalable Transparent Argument of Knowledge) are both cryptographic protocols that allow one party (the prover) to prove to another party (the verifier) that they know a certain piece of information, without revealing any information about the actual content of that information.

The key difference between the two protocols is that zk-SNARKs are based on elliptic curve cryptography, while zk-STARKs are based on polynomial commitments. This means that zk-SNARKs are generally faster and more efficient, but they also require a trusted setup phase in which the public parameters of the system are generated. This trusted setup can be a security concern, as it requires all parties involved to be present and to generate the parameters in a secure manner. In contrast, zk-STARKs do not require a trusted setup and are therefore considered to be more secure.

Both zk-SNARKs and zk-STARKs are used in a variety of applications, including anonymous transactions, confidential contracts, and voting systems. In all of these applications, the goal is to allow parties to prove that they have certain information without revealing the actual content of that information.

Groth '16 is a zkSNARK proof system that was proposed by Jens Groth in 2016. It is based on the use of pairing-based cryptography and is efficient in terms of proof size, but it requires a trusted setup phase in which the prover and verifier agree on certain parameters that are used in the proof.

Ben-Sasson '18 is a zkSNARK proof system that was proposed by Eli Ben-Sasson and his team in 2018. It is based on the use of linear algebra and is efficient in terms of proof size and prover time, but it requires a trusted setup phase similar to Groth '16.

PLONK (Purely Linear SNARK) is a proof system that was proposed by the team at ChainSafe Systems in 2019. It is based on the use of linear algebra and does not require a trusted setup phase, making it more transparent than systems like Groth '16 and Ben-Sasson '18. It is efficient in terms of proof size and prover time, but it requires the use of a specific type of constraint system called a "quadratic arithmetic program" (QAP).

All of these proof systems have been widely studied and have been used in various applications, including in the development of blockchain and cryptocurrency technologies. They have also inspired the development of other proof systems, such as zk-Rollup and zk-Sync, which aim to improve upon the efficiency and scalability of these systems.


zk-SNARKs protocol:

![image](https://user-images.githubusercontent.com/117555665/208236100-45220589-e547-4888-b013-f1274047f5f1.png)


zk-STARKs protocol:

![image](https://user-images.githubusercontent.com/117555665/208236120-3b4d7597-49d4-4e04-affa-afbbf281c772.png)
