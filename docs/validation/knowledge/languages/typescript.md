# TypeScript Validation

## Question And Applicability

What additional evidence is useful when JavaScript is authored through
TypeScript? Apply this guide to a confirmed TypeScript project and its exact
`tsconfig` graph. TypeScript guidance complements JavaScript and runtime
evidence; it does not replace either.

## Failure Modes And Evidence

Type declarations, inference, project references, module settings, included
files, ambient definitions, and compiler-version changes can admit or reject
different programs. The TypeScript documentation describes static checking
before execution and removal of types from emitted JavaScript.
[Source: typescript.docs-6#typescript-for-javascript](https://www.typescriptlang.org/docs/handbook/typescript-from-scratch.html)

Run the compiler against the intended project configuration. `noEmit` produces
type/static evidence without writing JavaScript, while a separate build is
needed when emitted artifacts are delivered.
[Source: typescript.docs-6#no-emit](https://www.typescriptlang.org/tsconfig/noEmit.html)

## Limitations, Cost, And Mutation

Static types are erased and cannot establish runtime values, external data,
host APIs, timing, side effects, or semantic correctness. Coverage depends on
included files, strictness, declaration quality, escape hatches, and compiler
version. Incremental compilation and build tools may write caches or output;
plain `--noEmit` should be selected when mutation is not part of the claim.

## Selection And Next Steps

Inspect `tsconfig` inheritance, include/exclude boundaries, project references,
module settings, and strictness. Use the
[TypeScript compiler guide](../tools/typescript-compiler.md) for command shape,
then combine with [JavaScript](javascript.md), the confirmed runtime, and
behavioral tests such as [Vitest](../tools/vitest.md).
