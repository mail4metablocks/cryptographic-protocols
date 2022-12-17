# cryptographic-protocols


Zk-SNARKs (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge) and zk-STARKs (Zero-Knowledge Scalable Transparent Argument of Knowledge) are both cryptographic protocols that allow one party (the prover) to prove to another party (the verifier) that they know a certain piece of information, without revealing any information about the actual content of that information.

The key difference between the two protocols is that zk-SNARKs are based on elliptic curve cryptography, while zk-STARKs are based on polynomial commitments. This means that zk-SNARKs are generally faster and more efficient, but they also require a trusted setup phase in which the public parameters of the system are generated. This trusted setup can be a security concern, as it requires all parties involved to be present and to generate the parameters in a secure manner. In contrast, zk-STARKs do not require a trusted setup and are therefore considered to be more secure.

Both zk-SNARKs and zk-STARKs are used in a variety of applications, including anonymous transactions, confidential contracts, and voting systems. In all of these applications, the goal is to allow parties to prove that they have certain information without revealing the actual content of that information.


zk-SNARKs protocol:

![image](https://user-images.githubusercontent.com/117555665/208236100-45220589-e547-4888-b013-f1274047f5f1.png)


zk-STARKs protocol:

![image](https://user-images.githubusercontent.com/117555665/208236120-3b4d7597-49d4-4e04-affa-afbbf281c772.png)
