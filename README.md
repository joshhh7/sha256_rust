# Pure Rust implementation of SHA-256

> [!WARNING]  
> **This implementation is for educational purposes only.**  
> It  has not been fully tested and audited, other than compliance testing.  
> Use it at your own risk.

`sha256_rust` is a pure Rust implementation of the hash function SHA-256. This implemenation was made with the intent of helping me understand and learn the Rust programming language. It is my first project in Rust.

The process of implementing SHA-256 was made easier with the step by step explanation of the algorithm in the following website : [Sha256 Algorithm Explained](https://sha256algorithm.com/).

Compliance testing was performed using the tool [crypto-condor](https://github.com/quarkslab/crypto-condor). crypto-condor tests implementations through test vectors. The dedicated harness is in the file [compliance_testing/sha_wrapper.py](https://github.com/joshhh7/sha256_rust/blob/master/compliance_testing/sha_wrapper.py). The file [compliance_testing/test_results2.txt](https://github.com/joshhh7/sha256_rust/blob/master/compliance_testing/test_results2.txt) contains the final results displayed by crypto-condor.

## Usage
To build the repository (cargo is required) :
```console
> cargo build
```

Run `src/main.rs`, then enter hexadecimal input :
```console
> $ cargo run
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
        Running `target/debug/sha256`
> ffff
ca2fd00fa001190744c15c317643ab092e7048ce086a243e2be9437c898de1bb
```

To run manual tests in `src/main.rs`, run :
```console
> $ cargo test
```

## Compliance testing
To run crypto-condor's test vectors, first build the project with the release profile :
```console
> $ cargo build --release
```

Make sure crypto-condor is installed in your machine, you can install it with the following command :
```console
> $ python -m pip install crypto-condor
```

Then run the following command which will run the wrapper file with the tool's test vectors for SHA-256 :
```console
> $ crypto-condor-cli test wrapper SHA compliance_testing/sha_wrapper.py
```
