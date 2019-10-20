# tokei-aas

Tokei as a Service

## Building and running

```bash
cargo run
```

## Testing

*   Run a mutation

    ```bash
    curl \
      -H "Accept: application/json" \
      -H "Content-Type: application/json" \
      -X POST \
      --data '{ "query": "mutation { add(by: 1) }" }' \
      http://127.0.0.1:8000/graphql
    ```

*   Run a query

    ```bash
    curl \
      -H "Accept: application/json" \
      -H "Content-Type: application/json" \
      -X POST \
      --data '{ "query": "{ accumulator }" }' \
      http://127.0.0.1:8000/graphql
    ```
