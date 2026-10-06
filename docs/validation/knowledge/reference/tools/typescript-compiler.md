# TypeScript Compiler

## Purpose And Scope

The project-local `tsc` executable parses TypeScript, loads one project graph,
performs static checks, and optionally emits JavaScript and declarations.

## Version Lines And Sources

This guide covers the maintained TypeScript 6 major line represented by the
registered documentation snapshot. Confirm the project-local compiler with
`pnpm exec tsc --version`. Review another major line separately when a workspace
supports it; the reviewed patch is provenance, not public identity.

## Command Model

Use the following representative non-emitting form, replacing the config path
with the intended project:

```text
pnpm exec tsc --noEmit --project <path-to-tsconfig>
```

The project option selects a configuration.
[Source: typescript.docs-6#project-option](https://www.typescriptlang.org/docs/handbook/compiler-options.html)
`noEmit` suppresses JavaScript and declaration output.
[Source: typescript.docs-6#no-emit](https://www.typescriptlang.org/tsconfig/noEmit.html)

## Inputs Outputs And Exit Status

Inputs include the selected `tsconfig`, referenced projects, included source,
declarations, module resolution, compiler version, and environment. Diagnostics
and nonzero status report configuration, syntax, or type failure.

## Mutation Cost And External Services

`--noEmit` prevents intended program output, but incremental configuration,
wrappers, plugins, and package-manager invocation can write caches. The compiler
does not inherently require a network service once dependencies exist.

## Evidence Produced

A pass supports the claim that the exact project graph satisfies that compiler
line's configured static checks without emission.

## Blind Spots

Declared types can be wrong or unsound. Static checking does not establish
runtime values, external contracts, host support, package contents, or semantic
correctness. Unselected projects and files remain outside the observation.

## Related Knowledge

Use [TypeScript Validation](../../explanations/languages/typescript.md),
[JavaScript Validation](../../explanations/languages/javascript.md), [pnpm](pnpm.md),
and runtime tests such as [Vitest](vitest.md) when applicable.
