# Phase B05.5: Publish .NET Bindings

## Status

`Future`. Blocked by an approved B05.0 .NET scope.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Publish NuGet package `liknon` only when the approved idiomatic managed .NET
API and a representative consumer are ready.

## Design Gate

When the selected API requires native core behavior, prototype the native
boundary and package shape against:

- supported target frameworks and runtime identifiers;
- P/Invoke or another approved interop mechanism;
- safe handles, memory ownership, callbacks, cancellation, and async behavior;
- managed exception and result types;
- RID-specific native assets and package resolution;
- trimming, single-file deployment, and native-library loading where supported.

When no selected capability requires native code, do not add an interop layer
merely for symmetry. The NuGet package is a `PackageReference`, not a .NET
Tool. RID assets contain native library payloads for the API and never the
Liknon CLI executable.

If B05.0 selects Git-hook integration, expose idiomatic managed plan, diff,
apply, verification, and diagnostic types over the shared
[integration contract](git-hook-integration.md). Restoring or loading the
package must not mutate hook configuration, and generated hooks invoke the
selected native Validation command directly rather than start .NET as a
forwarder.

## Verification

- Representative managed APIs pass shared conformance fixtures.
- Native assets resolve correctly for every declared RID.
- Safe-handle and callback lifetimes survive cancellation and garbage
  collection.
- Exceptions preserve the approved taxonomy and no native panic crosses the
  boundary.
- A consumer project builds, publishes, and runs without invoking the CLI.
- The package contains no CLI executable or authored Hub resource payload.
- Approved hook adapters pass shared preservation, idempotency, explicit-apply,
  and absent-CLI fixtures.

## Exit Criterion

.NET applications reference Liknon as a managed library with safe native
interop and no global tool installation.
