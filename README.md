This application consists of a server and a client.
Server provides quotes (wisdom) upon the client's requests.
However, the server requires the client upon each request to solve a challenge to protect
itself from DoS attacks. This challenge is a simplistic PoW version demanding to find a nonce
such that the challenge together with the nonce hash out to a value satisfying some difficulty chosen by
the server. Difficulty is a number of leading zeros in SHA256 hash.

Run the server
```bash
cargo r --release --bin server
```

run the client
```bash
cargo r --release --bin client
```