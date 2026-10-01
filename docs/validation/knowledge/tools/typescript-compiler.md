# TypeScript Compiler

## Identity And Reviewed Scope

- Executable/package: `tsc` from the `typescript` package, normally invoked as
  a project-local binary.
- Reviewed behavior: TypeScript 6.0.3 documentation snapshot.
- Evidence class: parsing, project configuration, type checking, and optionally
  JavaScript/declaration emission.

## Useful Command Structure

```text
pnpm exec tsc --noEmit --project tsconfig.json
```

The project option selects the configuration to compile, while `noEmit`
suppresses JavaScript and declaration output.
[Source: typescript.docs-6#project-option](https://www.typescriptlang.org/docs/handbook/compiler-options.html)
[Source: typescript.docs-6#no-emit](https://www.typescriptlang.org/tsconfig/noEmit.html)

## Output, Exit, Cost, And Mutation

Diagnostics identify configuration, syntax, and type errors; a failed compile
returns nonzero. `--noEmit` avoids intended artifact output, although compiler
configuration and wrappers must still be inspected for incremental/cache
behavior. Full builds can write JavaScript, declarations, maps, and build info.

## Blind Spots And Next Step

Type checking cannot establish runtime values, external contracts, semantic
correctness, or host support. Its coverage follows the exact configuration and
declarations. Pair it with [TypeScript guidance](../languages/typescript.md),
runtime tests, and artifact build/inspection when emitted output ships.
