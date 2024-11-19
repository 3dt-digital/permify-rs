### ⚠️ Warning: Under Development

**This project is currently under active development.**  
Features and API endpoints may not yet be fully implemented, and breaking changes can occur at any time. While we strive to provide a stable and functional client, please note the following:

1. **Incomplete Functionality:** Some API endpoints might still be missing or partially implemented.
2. **Breaking Changes:** Function signatures, structures, and behavior may change without prior notice.
3. **Limited Testing:** The client has not yet undergone comprehensive testing. Bugs or unexpected behavior might occur.

If you encounter issues, please feel free to [open an issue](https://github.com/3dt-digital/permify-rs/issues) or contribute improvements.

We recommend using this library with caution in production environments until it reaches a stable release.

---

### 🎯 Roadmap to Stability

1. Complete implementation of all Permify API endpoints.
2. Add comprehensive test coverage.
3. Provide detailed examples and documentation for usage.
4. Release version `1.0.0` after stability and reliability are achieved.

---

# Permify Client

A Rust client library for interacting with the [Permify API](https://docs.permify.co/api-reference/introduction). This library provides a simple and efficient way to perform authorization checks and manage permissions using Permify's REST API.

## Features

- Perform role-based access control (RBAC) checks
- Retrieve projects and related entities
- Flexible and easy-to-use API client
- Built-in support for authentication with API keys
- Asynchronous requests powered by `reqwest` and `tokio`

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
permify-rs = { git = "https://github.com/3dt-digital/permify-rs", branch = "main" }
```

## Requirements

- Rust 1.65 or higher
- API key for the Permify API
- dotenv for managing environment variables (optional)

## Usage

### Setup

Make sure you have a .env file with the following variables:

```env
PERMIFY_API_KEY=your_api_key_here
PERMIFY_BASE_URL=https://api.permify.co/v1/tentants/your_tenant_id_here 
```

### Example Code

```rust
use permify_client::{PermifyClient, RoleCheckRequest};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("PERMIFY_API_KEY")?;
    let base_url = env::var("PERMIFY_BASE_URL")?;

    let client = PermifyClient::new(&base_url, &api_key);

    // Example: Fetch projects
    let projects = client.get_projects().await?;
    println!("Projects: {:?}", projects);

    // Example: Role Check
    let role_check_request = RoleCheckRequest {
        subject_id: "user123".to_string(),
        relation: "admin".to_string(),
        object: "project:456".to_string(),
    };

    let role_check_response = client.role_check(role_check_request).await?;
    println!("Role Check: {:?}", role_check_response);

    Ok(())
}
```

### Features in Development

- Full support for all Permify API endpoints
- Enhanced error handling and retries
- Automatic pagination for large datasets
- gRPC support for faster and more efficient communicationgib mir 

## Contributing

Contributions are welcome! Please follow these steps to contribute:
1. Fork the repository
2. Create a new branch for your feature or bugfix (`git checkout -b feature/your-feature`).
3. Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## References

- [Permify API Documentation](https://docs.permify.co/api-reference/introduction)
- [Rust Reqwest Documentation](https://docs.rs/reqwest)
- [Rust Tokio Documentation](https://docs.rs/tokio)
```
