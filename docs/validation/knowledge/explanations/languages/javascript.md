# JavaScript Validation

## Scope

This guide covers JavaScript language syntax and runtime semantics. ECMAScript
defines the language independently from host facilities such as files,
processes, modules, browsers, and Node.js APIs.
[Source: ecma.ecmascript-2025#language-overview](https://262.ecma-international.org/16.0/#sec-overview)

## Failure Modes

Material failures include syntax errors, coercion and equality mistakes,
incorrect asynchronous ordering, exception and rejection handling, mutation or
aliasing bugs, boundary-value errors, and assumptions about host-provided
globals. Build transformations can introduce behavior that differs from the
authored source.

## Validation Layers

Use parsing or static analysis for selected language defects, unit tests for
local semantics, integration tests for real module and collaborator behavior,
and runtime/platform tests for each promised host. Inspect emitted or bundled
JavaScript when consumers execute transformed output.

## Evidence And Limitations

A JavaScript test establishes only the selected inputs, oracle, host,
configuration, and transformed artifact. It cannot establish TypeScript type
coverage, browser compatibility, Node.js support, package correctness, or
unexercised asynchronous schedules. Test execution may write snapshots, caches,
coverage, or other configured output and may contact external services.

## Version Scope

This guide uses ECMAScript 2025 terminology because the language edition is
intrinsic source identity. Consumers must confirm that their parser, target,
transpiler, and host implement the features they rely on; one host patch does
not define JavaScript applicability.

## Related Tools And Recipes

Use [Node.js Runtime Evidence](../technologies/nodejs.md) for host behavior,
[TypeScript Validation](typescript.md) for typed source, [Vitest](../../reference/tools/vitest.md)
for runner behavior, and the [JavaScript/TypeScript Package Gate](../../how-to/recipes/javascript-typescript-package-gate.md)
for one adaptable composition.

## Next Step

State the language-level failure and observable first, then select only the host,
tool, and artifact layers needed by the product promise.
