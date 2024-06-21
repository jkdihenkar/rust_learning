# grpc-calculator

A simple Add calculator built using the gRPC in rust with tonic.

Some gRPC helper utilities:
- grpcurl (`brew install grpcurl`)
- grpcui (`brew install grpcui`)


## `grpcurl` usages
With proto:
```sh
> grpcurl -plaintext  -proto ./proto/calculator.proto -d '{"a": 1, "b": 2}' 'localhost:50051' calculator.Calculator.Add
{
  "result": "3"
}
```

Without proto reference when server supports reflection:
```sh
> grpcurl -plaintext  -d '{"a": 3, "b": 2}' 'localhost:50051' calculator.Calculator.Add
{
  "result": "5"
}
```

List with grpcurl:
```sh
> grpcurl -plaintext 'localhost:50051' list
calculator.Calculator
grpc.reflection.v1alpha.ServerReflection
```

## `grpcui` usages

Invoke the gRPC UI on the server - 
```sh
❯ grpcui -plaintext 'localhost:50051'
gRPC Web UI available at http://127.0.0.1:61153/
```

UI for request:
![request-ui](assets/image.png)

Response on UI:
![response-ui](assets/image2.png)
