# Carrier Crab Roadmap

This document serves as the high-level roadmap for building a fully-featured, cross-platform Carrier Crab (OpenCollection API Client) driven by Rust.

## Phase 1: CLI & Core Parser (Current)
Our immediate goal is to build the engine without worrying about UI complexities.
- [ ] Initialize standard Rust binary.
- [ ] Implement the `OpenCollection` spec parser using `serde_yaml`.
- [ ] Build a CLI to read a `.yaml` file and parse it.
- [ ] Integrate `reqwest` to actually execute the API request and print the response to stdout.

## Phase 1.5: OpenCollection Standard Parity
- [ ] **PRIORITY 1**: Request Body Definitions (Support parsing and sending `json`, `xml`, `text`, `formUrlEncoded`, and `multipartForm` payloads).
- [ ] **PRIORITY 2**: Variables Hierarchy (Resolve variable values falling back sequentially through Request -> Folder -> Collection -> Environment -> Global `process.env`).
- [ ] Authentication Abstractions (Implement first-class `auth` blocks handling, encrypting, and building headers natively).
- [ ] Testing Assertions Evaluator (Parse and validate the JSON/HTTP responses against custom HTTP structural tests).
- [ ] Scripting Layers (Investigate and architecturalize embedding a Javascript runtime, e.g., `v8` or `boa`, for processing Pre-Request and Post-Response scripts).
- [ ] Additional Protocols (Explore structuring the config format to route to GraphQL/gRPC pipelines).

## Phase 2: Interactivity & Watching
- [ ] Integrate the `notify` crate to watch a `collections` directory for changes.
- [ ] Implement an interactive terminal prompt (like an SSH menu) or a lightweight TUI (Terminal UI using `ratatui`) to list and select endpoints to run without restarting the CLI.

## Phase 3: The GPUI Application
- [ ] Once the core parser, executer, and state management are rock solid, construct a `gpui` frontend package.
- [ ] Wire the core engine as a library to the GPUI interface.
- [ ] Deliver the blazingly fast native desktop app experience.
