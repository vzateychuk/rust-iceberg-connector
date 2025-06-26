# Rust CloudEvents to Iceberg Connector
This is my first RUST application, which API accept input data to store to Iceberg

## 📄 Project Requirements

### Project Name: 
Rust CloudEvents to Iceberg Connector

### Objective:

Develop a reusable Rust library that:
- Accept incoming CloudEvent messages,
- Ingest the payload into Apache Iceberg in real-time,
- Be language-agnostic (usable via FFI or APIs from Java, Kotlin, Go, etc.),
- Be serverless deployable on OpenShift or Kubernetes.

### Requirements:
Language: Rust (2021 edition).

Functionality:
- CloudEvents parsing (v1.0 spec compliance),
- Real-time ingestion into Apache Iceberg tables (via object store or catalog),
- Error handling & observability (logs, metrics).

Interoperability: Expose via FFI (Foreign Function Interface)  or REST/gRPC APIs for use in non-Rust systems (Java, Kotlin, Golang, etc.).

Deployment:
- Serverless compatible (OpenShift Functions, Knative, or K8s Jobs/Deployments),
- Container-ready (Dockerfile, Helm chart optional).

Testing:
- Comprehensive unit tests,
- End-to-end (e2e) tests simulating CloudEvent ingestion → Iceberg write.

## 🏗️ Architecture & Development Plan

1. High-Level Design
Input Layer:
- Accept CloudEvents over HTTP (using actix-web or warp) or Kafka (optional).

Event Parser:
- Deserialize CloudEvent (using cloudevents-sdk).

Data Transformer:
- Validate & transform payload to Apache Arrow or Parquet format.

Ingestion Engine:
- Write to Apache Iceberg (via REST catalog or object store, e.g., S3, GCS).

Interop Layer:
- FFI bindings (via cbindgen) or REST/gRPC wrapper.

Deployment:
- Containerized using Docker,
- Configurable for OpenShift/K8s with health probes.