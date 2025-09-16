# Cardea Calculator MCP Server

The Cardea Calculator MCP server provides basic arithmetic operations such as addition and subtraction. It supports two transport types: `stream-http` and `sse`.

## Tools

- **sum**
  - Calculate the sum of two integer numbers
  - Input parameters:
    - `a`: The first integer
    - `b`: The second integer
  - Returns the sum of the two integers

- **sub**
  - Calculate the difference of two integer numbers
  - Input parameters:
    - `a`: The first integer
    - `b`: The second integer
  - Returns the difference of the two integers

## Build

```bash
# build mcp server
cargo build --package cardea-calculator --release
```

## Run

Now, let's start the mcp server. You can choose to start the server with different transport types by specifying the `--transport` CLI option. The default transport type is `stream-http`. In addition, you can also specify the socket address to bind to by specifying the `--socket-addr` CLI option. The default socket address is `127.0.0.1:8001`.

```bash
# run mcp server (stream-http)
./target/release/cardea-calculator --transport stream-http

# run mcp server (sse)
./target/release/cardea-calculator --transport sse
```

If start successfully, you will see the following output:

```bash
Calculator MCP server is listening on 127.0.0.1:8001
```
