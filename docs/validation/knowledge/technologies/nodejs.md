# Node.js Runtime Evidence

## Question And Applicability

What runtime evidence is required for JavaScript or emitted TypeScript that
promises Node.js support? Confirm the supported Node.js major versions,
operating systems, module contract, native dependencies, and environment before
applying this guide.

## Failure Modes And Evidence

Runtime failures can arise from unsupported syntax/APIs, CommonJS versus ESM
interpretation, exports, environment variables, filesystem/process behavior,
native addons, and platform differences. Node.js package metadata participates
in deciding how `.js` files and package entry points are interpreted.
[Source: nodejs.api-22#modules](https://nodejs.org/docs/v22.16.0/api/packages.html)

Direct evidence includes executing tests and built entry points on each
promised Node.js major/platform combination, exercising module boundaries, and
checking documented failure exit behavior. Node.js assigns process exit codes
to ordinary and runtime-specific termination conditions.
[Source: nodejs.api-22#exit-codes](https://nodejs.org/docs/v22.16.0/api/process.html#exit-codes)

## Limitations, Cost, And Mutation

One local Node.js version cannot establish a supported matrix. Tests may depend
on locale, time, filesystem, signals, permissions, or services. Package scripts
execute with the current user's authority and may mutate local or external
state.

## Selection And Next Steps

Derive a minimal version/platform matrix from product promises and dependency
engines. Combine [JavaScript](../languages/javascript.md),
[TypeScript](../languages/typescript.md) when present, and the exact package
manager/test-runner guides. Record excluded hosts and native dependencies.
