# Phase B03.3: Prepare Tool Source And Target Evidence

## Status

`Repository implementation present`. Existing packaging and target work is
retained as focused evidence. Public source and native distribution belongs to
B04.

[Back to Plan B03](README.md)

## Objective

Prepare the Tool implementation, source closure, machine contracts, and
target-specific smoke evidence required for `liknon-cli` to own the public
`liknon-tool` executable.

## Handoff Contract

Record:

- exact Tool implementation sources and supporting-crate dependencies;
- direct command `liknon-tool`;
- launcher routes `liknon tool` and `liknon t`;
- absence of a physical `likt` target;
- implemented command grammar and C01-governed pending bootstrap surface;
- human and machine output schemas;
- errors and exit codes;
- offline and read-only effect contract;
- local store discovery and explicit-workspace rules;
- semantic smoke commands for each B04 target;
- compatibility with the B02 resource-state contract.

No Tool-specific public version, tag, package, archive, R2 prefix, stable
pointer, credential, or recovery train survives this handoff.

## Source Closure

Prove that:

- `liknon-cli` can build Tool outside the repository;
- every dependency is publishable and exact;
- no Hub corpus, managed resource payload, fixture store, secret, or generated
  state enters the public package;
- the program-owned bootstrap contract, if activated by C01, is explicitly
  distinguished from installed resources;
- shared terminal behavior comes from `liknon-cli-ui`;
- shared resource identities come from `liknon-resource-contracts`;
- Tool-specific JSON, errors, and presentation remain Tool-owned.

## Target Evidence

For every target selected by B04:

1. start `liknon-tool` and verify the expected suite version;
2. run catalog, search, show, resolve, and context fixtures offline;
3. exercise valid Manager-produced state and contract-valid fixture state;
4. reject modified, missing, unsafe, incompatible, revoked, and unsupported
   state;
5. prove no network access or filesystem mutation occurs;
6. compare direct execution with both launcher namespace forms;
7. preserve argument, environment, working-directory, stream, signal, timeout,
   interruption, and exit-code behavior.

A failed Tool row blocks the complete B04 archive for that target.

## Acceptance Criteria

- [ ] Tool has one physical executable name: `liknon-tool`.
- [ ] `tool|t` exist only in the launcher parser.
- [ ] The source closure builds through `liknon-cli` without unpublished paths.
- [ ] Package inventory contains no authored Hub resource corpus.
- [ ] Every selected target has offline Tool smoke evidence.
- [ ] Direct and routed behavior are equivalent.
- [ ] Tool remains network-free, read-only, and independent from Manager process
  availability.
- [ ] The handoff contains no standalone Tool release identity.

## Exit

B03.3 exits with a complete, reproducible Tool handoff that B04 can compile,
package, and verify without inventing Tool behavior or a second release train.
