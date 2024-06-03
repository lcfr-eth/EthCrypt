# ETH-Crypter  

Use [ECIES](https://medium.com/asecuritysite-when-bob-met-alice/elliptic-curve-integrated-encryption-scheme-ecies-encrypting-using-elliptic-curves-dc8d0b87eaa) to encrypt text and files <= 1G using an ethereum private key.

# Usage
```
export PKey=key
cargo run
Usage: --test | --encrypt <public_key_hex> <message> | --decrypt <secret_key_hex> <encrypted_hex>

cargo run -- --encrypt 04d5cfb5490134260b600c42cd53ad7fbaba37ceb67ef9fe4a7fd7a64161980330c09900dd24a873bbb31179547e76727950a92502fd1ac74e5ad6f5c5836e3179 helloworld`
Message encrypted: 04b02718c9de97266b3ce78529ffe6c3c5a62794a8c4f5dc287278e38f51425746d23470acdff1fa4a26bb6606d69db64fc0295081ca786d78b9553d6036c2cc8c7525fd34bc18ca00597bc161e1cb21a9b1f05c7aead6ef0bb30bc1dce938f86973670abd2f32d1f31938

cargo run -- --decrypt 0445060e3378dcd49e15f3320516891230ca7514426401ce5e29455ce75360439d59e054a08b6d41a0d9c6afe9adce11eccb980dfbff57fd0690a8fb830134908ed1c81f6ce4684e2ed842675a342bf2b3c1c328d7ce7e385a939f13c00d7845a41c272c53556d978a114d
Decrypted message: helloworld

cargo run -- --encrypt-file 04d5cfb5490134260b600c42cd53ad7fbaba37ceb67ef9fe4a7fd7a64161980330c09900dd24a873bbb31179547e76727950a92502fd1ac74e5ad6f5c5836e3179 test1 test1.enc

cargo run -- --decrypt-file test1.enc outfile
Decryption complete. Decrypted data written to: outfile
```