# TypeScript Validation

## Scope

This guide covers static evidence added when JavaScript is authored through
TypeScript. TypeScript checks source before execution and erases types from
emitted JavaScript. [Source: typescript.docs-6#typescript-for-javascript](https://www.typescriptlang.org/docs/handbook/typescript-from-scratch.html)

## Failure Modes

Failures include incorrect type relationships, unresolved modules or
declarations, unintended project inclusion or exclusion, incompatible compiler
options, unsound assertions, declaration drift, and emitted output that does
not match the selected runtime or package contract.

## Validation Layers

Check the exact project configuration without emission for static evidence.
Build and inspect emitted declarations or JavaScript when they ship. Execute
runtime tests against the consumed artifact and promised host. Validate
multiple projects separately when references or configurations change scope.

## Evidence And Limitations

Type checking uses declared types and configured files; it cannot establish
runtime values, external service contracts, host compatibility, or semantic
correctness. `noEmit` supports a non-emitting check, but wrappers and
incremental settings may still create caches.
[Source: typescript.docs-6#no-emit](https://www.typescriptlang.org/tsconfig/noEmit.html)

## Version Scope

Use the maintained TypeScript major line declared by the workspace and confirm
the project-local compiler with `tsc --version`. Exact reviewed patches remain
source provenance unless a patch materially changes a described decision.

## Related Tools And Recipes

Use the [TypeScript Compiler](../../reference/tools/typescript-compiler.md) for
command behavior, [JavaScript Validation](javascript.md) for emitted semantics,
[Node.js Runtime Evidence](../technologies/nodejs.md) for host support, and the
[JavaScript/TypeScript Package Gate](../../how-to/recipes/javascript-typescript-package-gate.md)
as one adaptable composition.

## Next Step

Identify the exact `tsconfig` boundary and consumed artifact, then pair static
evidence with the runtime and package evidence required by the product promise.
