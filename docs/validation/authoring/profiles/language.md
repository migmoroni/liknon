# Language Profile

- Kind: `language`
- Family: `explanation`
- Editorial bases: `oasis-dita-1.3-errata-02`, `diataxis`, `google-developer-documentation-style-guide`, `microsoft-writing-style-guide`

## Purpose And Basis

A language guide explains validation-specific failure surfaces in a programming
language. It uses DITA concept, Diataxis explanation, and shared terminology and
scannability guidance.

## Structure

Use Scope, Failure Modes, Validation Layers, Evidence And Limitations, Version
Scope, Related Tools And Recipes, and Next Step. All are required.

## Depth Version And Example

Separate language semantics from runtimes, compilers, package managers,
frameworks, and build tools. Discuss maintained editions or major lines only
when distinctions change decisions. A Rust guide may explain compiler-visible
failure surfaces but routes Cargo commands to the tool reference.

## Review Checklist

Confirm semantic scope, distinct failure modes, layered evidence, explicit
limits, useful version distinctions, correct routing, operational effects, and
no ecosystem overview. Apply the shared review rubric.
