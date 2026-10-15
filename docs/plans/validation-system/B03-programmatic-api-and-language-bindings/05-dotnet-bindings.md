# Phase B03.5: Publish .NET Bindings

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Expose the approved Liknon programmatic contract through an idiomatic managed
.NET library referenced by applications.

## Design Gate

Prototype the native boundary and package shape against:

- supported target frameworks and runtime identifiers;
- P/Invoke or another approved interop mechanism;
- safe handles, memory ownership, callbacks, cancellation, and async behavior;
- managed exception and result types;
- RID-specific native assets and package resolution;
- trimming, single-file deployment, and native-library loading where supported.

The NuGet package is a `PackageReference`, not a .NET Tool. RID assets contain
native library payloads for the API and never the Liknon CLI executable.

## Verification

- Representative managed APIs pass shared conformance fixtures.
- Native assets resolve correctly for every declared RID.
- Safe-handle and callback lifetimes survive cancellation and garbage
  collection.
- Exceptions preserve the approved taxonomy and no native panic crosses the
  boundary.
- A consumer project builds, publishes, and runs without invoking the CLI.

## Exit Criterion

.NET applications reference Liknon as a managed library with safe native
interop and no global tool installation.
