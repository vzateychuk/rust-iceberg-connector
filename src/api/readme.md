# 📁 'api' folder structure/files

```graphql
src/
├── api/
│   ├── mod.rs       # declares API modules
│   ├── rest.rs      # HTTP routes and handlers
│   └── grpc.rs      # optional gRPC service definitions
```

## mod.rs
- Defines and exposes all modules inside the api/ folder.
- Required for Rust to compile and use api::rest, api::grpc, etc.
- Clean way to organize your REST/gRPC interfaces separately from core logic.
