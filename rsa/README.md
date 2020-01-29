# RSA Toy *(Assignment 2)*

### Post Mortem

The.

### General Usage

#### Command-Line Demo:

 * `$` `cargo run --example demo <message>`

#### Unit Tests:

 * `$` `cargo test`

### Report:

I read the assignment directions on how to generate an RSA key and do the encryption/decrpytion and quickly realized that the way it was written in the assignment was wrong. So I diverted some time to deriving the correct solution myself, *(internet research, mostly)*. Once that was done, implementation was fairly easy. I noticed at multiple points there was a need to take a number and scale it up/down in bit size, so to fix this and avoid the need to have more try_from's and unwraps all over my code, I made two small utility functions that do the functional-like chaining to hide the messiness. For instance, I noticed that while p and q in the key wouldn't overflow by themselves *(in intended usage)*, the product of the two would produce an overflow, *(The product of pq for modulo, i.e. the public key)*. So to fix this, I stored their product in a u64, which means that p and q also needed to temporarily be u64's to do the operation and make rust happy. Other than some of those minor issues, I also took the time to make read/write functions so that my rsa_key interface could be stored on disk like real rsa keys. This was more just for fun, and a good excuse to learn file io as well, so now, if I desired, I could distribute my public key file for someone else to encrypt messages with and send them to me. Likewise,  I could have a server that sits and listens for messages *(ciphers)* and decrypt them, making a nice little model of real world usage.
