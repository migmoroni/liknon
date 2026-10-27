# Cross-Cutting Git Hook Integration Contract

## Status

`Future`. This candidate capability must be approved as part of Phase B05.0
before any language package exposes it.

[Back to the programmatic API and bindings plan](README.md)

## Purpose

Give each activated `liknon` language package an idiomatic way to integrate the
Validation CLI into an existing Git-hook workflow without making Liknon a hook
manager and without coupling normal hook execution to a language runtime.

The package performs explicit configuration work. The generated hook invokes a
public native Validation command directly, so Node.js, Ruby, Python, .NET, or a
binding runtime is not started for every commit merely to forward the command.

Local hooks are an early-feedback mechanism. They are bypassable and never
replace the repository's authoritative CI validation.

## Product Boundary

- `liknon-validator` owns validation execution and its public command grammar.
- Lefthook, pre-commit, and future providers continue to own hook scheduling
  and lifecycle.
- Package `liknon` exposes planning and configuration APIs for supported
  providers.
- The package does not bundle, install, update, or silently execute a Liknon
  CLI.
- Package installation, import, build, and lifecycle scripts never mutate Git
  hooks or provider configuration.
- An explicitly requested operation may discover or probe a configured CLI
  route and return structured compatibility evidence. It may not use CLI output
  as transport for unrelated programmatic Liknon behavior.

## Provider-Neutral Model

B05.0 must define one shared model before implementing a provider adapter. It
must represent at least:

- explicit workspace root and provider identity;
- hook stage and stable integration identity;
- explicit Validation command route, validation target, arguments, and
  environment additions;
- source-file selectors or provider-native filters when requested;
- existing configuration observations;
- proposed file operations and a human-readable and machine-readable diff;
- warnings, compatibility requirements, and verification results;
- ownership markers sufficient to update only the integration managed by the
  caller.

Provider adapters translate this model to provider-native configuration. They
must not redefine validation semantics or invent different safety rules in
each language binding.

## Explicit Lifecycle

Every mutating integration follows the same public lifecycle:

1. inspect the workspace, selected provider, and relevant existing files;
2. resolve an explicit CLI route and check compatibility only when requested;
3. build a deterministic change plan without mutation;
4. expose the exact diff and diagnostics to the caller;
5. apply only after a separate explicit call or approval supplied by the host;
6. write atomically and verify the resulting provider configuration;
7. return structured evidence describing changed, unchanged, and rejected
   files.

Planning is the default. A convenience API cannot collapse planning and
mutation into an implicit package-installation side effect. Reapplying the
same requested integration must be idempotent.

## CLI Resolution And Installation Guidance

- The caller may provide an exact executable path or an approved public command
  route.
- The canonical direct route is `liknon-validator`; an alias or the unified
  launcher route is used only when selected explicitly.
- Discovery searches only the documented environment and paths authorized by
  the caller.
- A version probe is an explicit process effect and returns typed evidence.
- An absent or incompatible CLI produces structured diagnostics and links or
  identifiers for valid B01 or B04 installation channels.
- The package never downloads the CLI, modifies `PATH`, installs another
  package, or chooses an installation channel on the caller's behalf.

Generated hook commands use explicit argument vectors wherever the provider
supports them. If a provider requires a command string, its adapter owns
platform-correct encoding and rejects values it cannot represent safely.

## Configuration Safety

- Parse structured provider files with an appropriate parser rather than text
  substitution.
- Preserve unrelated keys, hooks, comments, ordering, and formatting whenever
  the selected format and parser permit it.
- Refuse ambiguous ownership, malformed input, unsafe paths, symlink escapes,
  duplicate managed identities, and unsupported provider versions before
  mutation.
- Keep all writes within the explicit workspace and use atomic replacement.
- Never overwrite an existing hook command merely because its stage matches.
- Removal deletes only the exact managed integration and leaves unrelated
  provider state intact.
- Never run repository scripts or generated hooks while inspecting, planning,
  applying, or verifying configuration.

## Hook Strategy

The caller selects which validation target belongs to each hook stage. The
library may provide documented examples, but it must not hard-code project
group or suite IDs such as `fast`.

The initial provider candidates are:

- Lefthook;
- pre-commit using a repository-local/system-language hook where appropriate.

Additional providers implement the same contract and conformance fixtures.
Liknon does not replace an existing hook manager, install a second manager
automatically, or claim exclusive ownership of `.git/hooks`.

## Performance Claims

Native Validation startup can avoid booting a host-language runtime on every
hook, but configured checks normally dominate total hook duration. Public
documentation may publish startup or end-to-end timing only from a retained,
reproducible benchmark with its platform, version, cache state, and workload.
The plan does not assume or promise a fixed threshold such as 50 ms.

## Cross-Language Contract

Every activated language package exposes idiomatic host-language types over
the same observable behavior:

- equivalent plans for the same fixture and request;
- equivalent rejection and diagnostic categories;
- the same explicit mutation boundary;
- equivalent preservation and idempotency guarantees;
- no hidden CLI installation or validation execution.

Implementation may be shared through an approved core, stable data contract,
or native bridge. It must not be copied into divergent language-specific
implementations without common fixtures proving parity.

## Verification

- clean creation, merge into existing configuration, update, no-op reapply,
  removal, and malformed-input fixtures pass for every provider;
- unrelated hooks and formatting-sensitive content remain intact;
- plan mode performs no writes or process execution unless an explicit version
  probe was requested;
- apply mode writes only the reviewed plan and detects intervening changes;
- absent and incompatible CLIs return actionable structured guidance without
  downloading anything;
- generated hooks invoke the selected public Validation route directly;
- setup never executes a generated hook or arbitrary repository command;
- provider fixtures pass on every supported host platform;
- bypassing a local hook cannot bypass the documented CI validation gate.

## Exit Criterion

At least one provider adapter and one representative consumer prove safe
planning, explicit application, preservation, idempotency, direct native CLI
execution, and CI coexistence. Other language packages may expose the
capability only through the approved shared contract and fixtures.
