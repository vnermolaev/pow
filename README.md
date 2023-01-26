This application consists of a server and a client.
Server provides quotes (wisdom) upon the client's requests.
However, the server requires the client upon each request to solve a challenge to protect
itself from DoS attacks. This challenge is a simplistic PoW version demanding to find a nonce
such that the challenge together with the nonce hash out to a value satisfying some difficulty chosen by
the server. Difficulty is a number of leading zeros in SHA256 hash.

Run the client-server interaction
```bash
docker-compose up
```

The server will start and keep running until explicitly terminated. The client will make a single request and
terminate. To make another request, run in another terminal window
```bash
docker start pow-client-1
```
Observe the output in the terminal where containers have been built.