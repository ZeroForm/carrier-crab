# Carrier Crab 🦀

**Status Validation:** *Work in Progress (WIP)* 🚧

Carrier Crab is an experimental, barebones **OpenCollection** API client written in Rust. We're currently in the early, highly volatile phases of development. Right now, it's essentially just a command-line utility capable of stringing together HTTP requests.

*Do not use this for production testing (yet).*

## Features

Presently, Carrier Crab focuses on the execution of raw API requests derived from standard file specs:

- **YAML parsing**: Parses basic OpenCollection endpoint formats detailing HTTP verb, target, and standard Header definitions.
- **Environment Targeting**: Support for loading arbitrary scoped environments using `--env targetName`, pulling variable structures dynamically from `environments/targetName.yml`. 
- **Secret Subinjections**: Fully integrates local `.env` definitions to safely abstract and securely query passwords, tokens, or JWTs out-of-band via `{{process.env.SECRET_NAME}}` format.
- **Execution Logging**: Basic terminal printing of HTTP responses.

## Usage

```bash
# Execute a raw API file request 
cargo run -- example.yml

# Execute a request with local sandbox environment variables
cargo run -- example.yml --env local
```
