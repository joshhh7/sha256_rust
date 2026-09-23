# Pure Rust implementation of sha256

> [!WARNING]  
> **This implementation is for educational purposes only.**  
> It  has not been fully tested and audited, other than compliance testing.  
> Use it at your own risk.

`sha256_rust` is a pure Rust implementation of the hash function sha256. This implemenation was made with the intent of helping me understand and learn the Rust programming language. It is my first project in Rust.

The process of implementing *sha256* was made a lot easier with the step by step explanation of the algorithm in the following website : [Sha256 Algorithm Explained](https://sha256algorithm.com/).

Compliance testing was performed using the tool [crypto-condor](https://github.com/quarkslab/crypto-condor). The dedicated harness is in the file [compliance_testing/sha_wrapper.py](https://github.com/joshhh7/sha256_rust/blob/master/compliance_testing/sha_wrapper.py). The file [compliance_testing/test_results2.txt](https://github.com/joshhh7/sha256_rust/blob/master/compliance_testing/test_results2.txt) contains the final results displayed by crypto-condor.