# Phase B03.6: Publish The .NET AI Package

[Back to the ecosystem AI packages plan](README.md)

## Purpose

Publish NuGet package `liknon` as a managed resource library.

## Package Contract

- Target the approved .NET baseline, initially .NET 10.
- Expose shared and .NET resource inventory and exact bytes.
- Provide managed explicit materialization with safe path and conflict rules.
- Publish as `PackageReference`, not a .NET Tool.
- Include no `DotnetToolSettings.xml`, command shim, or native executable.
- Reserve coherent namespaces for future B04 managed APIs.

## Verification

- `dotnet restore` and a representative consumer build succeed.
- Package inspection contains only approved managed code, metadata, and
  resources.
- No command becomes globally available.
- Materialization fixtures pass on supported runners.
- Public readback matches the retained `.nupkg`.

## Exit Criterion

.NET consumers reference `liknon` as a resource library without receiving a
tool, knowledge corpus, or native validation payload.
