# ETH-Crypter  

Use [ECIES](https://medium.com/asecuritysite-when-bob-met-alice/elliptic-curve-integrated-encryption-scheme-ecies-encrypting-using-elliptic-curves-dc8d0b87eaa) to encrypt text and files <= 1G using an ethereum public key. 

Decrypt using corresponding private key.

# Usage
Requires the Public Key of an ethereum account to encrypt the data with.

Get PublicKey: 
```
export PKey=yourkey  
cargo run -- --pubkey
Public Key in Hex: 0419792b85ebdc61aadfe28767a516f3c807a38fc94fad34043c569fcc97ae2944b8c06592a5f84b1b39f611b430140a03af3ca94ab0d738cfc0af6b0bcb44e5ba
```

Now supply the ```PublicKey``` as an argument to the ```--encrypt``` functions as shown below:

```
export PKey=yourkey  

cargo run -- --encrypt 0419792b85ebdc61aadfe28767a516f3c807a38fc94fad34043c569fcc97ae2944b8c06592a5f84b1b39f611b430140a03af3ca94ab0d738cfc0af6b0bcb44e5ba helloworld`
Message encrypted: 04b02718c9de97266b3ce78529ffe6c3c5a62794a8c4f5dc287278e38f51425746d23470acdff1fa4a26bb6606d69db64fc0295081ca786d78b9553d6036c2cc8c7525fd34bc18ca00597bc161e1cb21a9b1f05c7aead6ef0bb30bc1dce938f86973670abd2f32d1f31938
```  

To decrypt the encrypted data provide it as an argument to ```--decrypt```
```
cargo run -- --decrypt 0445060e3378dcd49e15f3320516891230ca7514426401ce5e29455ce75360439d59e054a08b6d41a0d9c6afe9adce11eccb980dfbff57fd0690a8fb830134908ed1c81f6ce4684e2ed842675a342bf2b3c1c328d7ce7e385a939f13c00d7845a41c272c53556d978a114d
Decrypted message: helloworld
```  
Similarly for files we need to pass the ```PublicKey``` to the ```--encrypt-file```
```
cargo run -- --encrypt-file 0419792b85ebdc61aadfe28767a516f3c807a38fc94fad34043c569fcc97ae2944b8c06592a5f84b1b39f611b430140a03af3ca94ab0d738cfc0af6b0bcb44e5ba test1 test1.enc
```

Finally to decrypt files: 
```
cargo run -- --decrypt-file test1.enc outfile
Decryption complete. Decrypted data written to: outfile
```