
## Project Goals
The primary goal of this project is to implement the AES-256 encryption algorithm in Rust from scratch.
I don't aim for an efficient implementation and mostly want to get comfortable with Rust.
The project aims to explore potential enhancements or modifications to the standard AES-256 algorithm,
experimenting with unique twists to further strengthen (?) its cryptographic properties or adapt it to specific use cases.\
This project will serve as mostly a learning exercise in cryptography and Rust, as well as a foundation for developing advanced encryption tools.

## Todos
- [X] Implement finite field class
- ~~[ ] Implement matrix class~~
- ~~[ ] Implement (efficient) matrix multiplication~~
- [ ] Implement class and method for converting a string to a block
- [X] Implement round key generation
- [ ] Fix the bug involving mix_columns
- [ ] Implement an input system for user input
- [ ] Add support and generation for initialization vector


- [X] `AddRoundKey`
- [X] `SubBytes`
- [X] `ShiftRows`
- [ ] `MixColumns`