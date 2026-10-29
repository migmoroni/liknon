# Liknon Resource Contracts

`liknon-resource-contracts` defines the versioned JSON contracts shared by the
Liknon Resource Hub, Manager, and read-only Tool. It contains no network client,
workspace mutation, command execution, or authored resource payload.

The public schemas are included under `schemas/`. Contract version changes are
intentional compatibility changes and must update the Rust types, schemas,
tests, and consuming products together.
