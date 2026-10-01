# Node.js Runtime Evidence

## Role And Scope

Node.js is a host runtime for JavaScript and emitted TypeScript. Package module
interpretation and host APIs are Node.js concerns distinct from ECMAScript
language semantics. [Source: nodejs.api-22#modules](https://nodejs.org/docs/v22.16.0/api/packages.html)

## Failure Modes

Failures include CommonJS/ES module mismatches, unsupported runtime APIs,
conditional export resolution, native add-on or operating-system differences,
environment and signal handling, process exit semantics, permissions, and
behavior that varies across declared Node.js major lines.

## Validation Surfaces

Run the consumed artifact on every materially promised major line and platform.
Exercise module loading, process startup and exit, filesystem or network
boundaries, and real dependencies needed by the claim. Inspect package metadata
and built output when module or export shape matters.

## Evidence And Limitations

Node.js documents process exit status as runtime behavior used by command-line
tools. [Source: nodejs.api-22#exit-codes](https://nodejs.org/docs/v22.16.0/api/process.html#exit-codes)

One local run cannot establish another major line, platform, architecture, or
external service. Tests may mutate caches, files, snapshots, or remote state;
record their environment, cost, network, and cleanup boundaries.

## Version Scope

The reviewed provenance covers Node.js 22. Apply the guide by maintained major
line and test the range promised by the workspace. Confirm the active runtime
with `node --version`; do not treat the reviewed patch as general applicability.

## Related Knowledge

Use [JavaScript Validation](../languages/javascript.md), [TypeScript Validation](../languages/typescript.md),
[pnpm](../../reference/tools/pnpm.md), and [Vitest](../../reference/tools/vitest.md)
for their distinct boundaries.

## Next Step

State the promised Node.js major lines and runtime behaviors, then select an
explicit matrix and artifact-level observations proportional to that promise.
