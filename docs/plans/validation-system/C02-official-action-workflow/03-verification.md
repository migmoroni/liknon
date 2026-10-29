# Phase C02.3: Verify The Official Action Workflow

## Status

`Provisional`. Execution depends on implemented C01 and its focused B03
bootstrap follow-up. Existing B02/B03 local contracts and the B04 suite
topology are fixed inputs.

[Back to Plan C02](README.md)

## Deterministic Verification

Version fixtures must prove that:

- selecting or reading AI-Tool does not activate the workflow;
- selecting the workflow loads only its exact locked dependency closure;
- a missing resource stops without network or Manager activity;
- every operation uses public deterministic interfaces and typed outputs;
- no operation mutates policy or creates workflow-history artifacts;
- no CLI branches on official workflow, skill, profile, or operation IDs;
- an external compatible operational skill can replace AI-Tool;
- successive bounded requests do not inherit authority or hidden state.

## CLI Suite Binding

Fixtures must also prove that:

- every Validator, Manager, or Tool capability used by the workflow is
  semantically equivalent through its direct executable and corresponding
  `liknon validator`, `liknon manager`, or `liknon tool` route;
- every bound executable reports the same exact suite version and expected
  component identity;
- a missing, substituted, or version-incompatible sibling executable fails
  before policy-controlled work, resource mutation, or validation begins;
- no descriptor treats `liknon` as a separate capability or operation owner.

## Forward Agent Trials

Use ordinary, ambiguous, blocked, and adversarial prompts for `policy`,
`config`, `run`, `triage`, and `audit`. Record operation selection, loaded
context, commands, policy decisions, mutations, reports, unsupported claims,
human interventions, and final evidence.

The D01 evaluation program owns comparative model scoring. C02 owns only the
fixtures and assertions needed to prove its own resource contract.

## Documentation

Publish human guidance for selecting the workflow and each operation, while
keeping agent instructions in resource files. Documentation links to C01 policy
semantics and public CLI references rather than copying them.

## Exit Criterion

The official workflow completes representative bounded tasks safely, remains
replaceable, and adds measurable guidance without becoming required for direct
human or direct-agent CLI use.
