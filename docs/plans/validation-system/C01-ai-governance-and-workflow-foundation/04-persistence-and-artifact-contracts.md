# Phase C01.4: Define Persistence And Artifact Contracts

## Status

`Provisional`. Generic semantics are defined here; deterministic operation
ownership is blocked by C01.0.

[Back to Plan C01](README.md)

## Purpose

Define append-only workflow evidence and exact validation-report persistence
without creating mutable workflow state, a general integrity registry, or
workflow-specific application branches.

## Separate Stores

```text
.liknon/
|-- validation/
|   `-- reports/
`-- ai/
    `-- persistence/
```

- validation reports retain exact machine-report bytes under the validation
  contract;
- workflow artifacts contain bounded analytical results declared by the exact
  locked workflow;
- neither store becomes executable validation configuration;
- neither store authorizes continuation merely because a file exists.

## Common Artifact Envelope

Every persistent workflow result must include:

- envelope and payload-schema versions;
- UUIDv7 artifact identity;
- creation timestamp;
- workspace identity without leaking credentials;
- workflow identity and exact locked version;
- node and action IDs;
- artifact declaration ID and kind;
- parent artifact and validation-report references where declared;
- bootstrap, policy, resource-lock, configuration, and application evidence
  required by the producing contract;
- payload digest, byte length, sensitivity classification, and repository
  treatment;
- bounded result payload.

Each completed process creates a new immutable artifact. Re-running a node does
not overwrite its previous result. Relationships express history; there is no
`state.json`, `state-ignore.json`, latest-pointer authority, or global hash
registry.

## Workflow Declarations

Each persistable artifact declaration must define:

- stable declaration and artifact-kind IDs;
- exact payload schema;
- producing node/action IDs;
- namespaced destination template below the workflow's allowed root;
- allowed relationships;
- byte, field, reference, and sensitivity limits;
- repository treatment of `local-ignored` or `consumer-selected`;
- immutable creation and explicit governed removal lifecycle.

Unknown treatments, boundary escapes, symlink escapes, undeclared executable
content, and attempts to relax bootstrap or policy fail closed.

## Policy Application

- `validationReportPersistence` governs requested storage of exact validation
  reports.
- `sensitiveDataPersistence` additionally applies whenever persisted content
  contains protected data.
- every other implicated category still applies to the concrete destination and
  operation;
- a permissive result in one category never cancels a restrictive result in
  another;
- no policy level permits an AI to mutate `policy.json`.

## Security And Lifecycle

- Persist only fields required by the declared schema.
- Never persist credentials, tokens, private keys, raw secret streams, or source
  patches unless an explicit contract and all applicable policy outcomes permit
  the exact bytes and destination.
- Use restrictive permissions where supported.
- Keep paths inside the selected workspace.
- Perform atomic append-only creation.
- Retain history until an explicit governed removal.
- Do not add automatic expiry, pruning, compaction, or replacement in the
  initial contract.

## Exit Criterion

Any compatible workflow can declare and validate immutable history through one
generic envelope, while report evidence remains exact and separate and no
mutable state subsystem is introduced.
