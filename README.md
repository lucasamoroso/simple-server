# Simple Rpc server with Tonic

## Run Server

```#bash
cargo run -- --port=8080
```

## Request

```#bash
grpcurl -plaintext -import-path ./api/protos \
    -proto ./api/protos/greeter.proto \
    -d '{"name": "Tonic"}' \
    127.0.0.1:8080 \
    greeter.Greeter/SayHello
```
