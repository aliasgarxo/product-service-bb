
# Product Service

This is a Rust-based API for the Best Buy product catalog. It allows employees to manage product inventory and customers to view product details.

## Running the app locally

The app runs independently of other services, so you can test it directly.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Running the app

Clone the repository, navigate to the `product-service` directory, and execute:

```bash
cargo run
```

When the app starts, you’ll see:

```text
Listening on http://0.0.0.0:3002
```

Test the API using the `test-product-service.http` file in VS Code with the REST Client extension.
    