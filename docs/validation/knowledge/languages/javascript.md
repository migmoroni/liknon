# JavaScript Validation

## Question And Applicability

What language-level evidence is useful for JavaScript source? Apply this guide
after confirming that JavaScript is authored or emitted and identifying its
ECMAScript edition, module form, strictness, and execution hosts. Do not use it
as a substitute for host-runtime, browser, framework, or TypeScript guidance.

## Failure Modes And Evidence

JavaScript parsing and execution can fail because syntax, module assumptions,
dynamic values, coercion, asynchronous behavior, or host-provided APIs differ
from expectations. ECMA-262 specifies the language while leaving host-defined
facilities to an embedding environment.
[Source: ecma.ecmascript-2025#language-overview](https://262.ecma-international.org/16.0/#sec-overview)

Language evidence can include parsing/bundling under the promised edition,
focused unit tests over observable values and failures, property tests for
input invariants, and runtime tests in each supported host. When JavaScript is
generated, inspect or execute the emitted artifact rather than assuming that a
successful source-level check establishes emission or loading.

## Limitations, Cost, And Mutation

A test runner observes only selected inputs, configuration, timers, module
resolution, and host. A parser does not establish behavior. Node.js results do
not establish browser compatibility, and a browser simulation does not prove
every deployed browser. Build and test tools may generate caches, coverage, or
bundles and may execute dependency code.

## Selection And Next Steps

State the promised hosts and module contract first. Add
[TypeScript](typescript.md) only when typed source is present, and load a host
guide such as [Node.js](../technologies/nodejs.md) only when that runtime is
confirmed. Use [oracles and failure paths](../patterns/oracles-and-failure-paths.md)
to select bounded tests.
