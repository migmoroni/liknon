# Configuration Reference

The exact configuration contract is version 6 and follows:

```text
tool -> check -> suite -> group
```

Tools own executable discovery and version requirements. Checks own reusable
argument templates and timeouts. Suites own contextual parameters, check
dependencies, and working directories. Groups own ordered references to suites
and nested groups. Shared nodes execute once per run.

The complete document is rejected for unknown fields, unsupported versions,
invalid or duplicate IDs, missing references, tool/group cycles, late or
missing dependencies, invalid parameters, and unsafe directories. A placeholder
that occupies one complete argument expands to zero, one, or several literal
arguments; values are never split or interpreted by a shell. An omitted direct
check placeholder expands to no argument.

`workspaceRoot` establishes the sole filesystem boundary. Suite directories
must be relative, existing directories contained below it; absolute paths,
parent traversal, files, and symlink escapes are rejected.

Configuration is trusted executable policy. Review changes with the same care
as source code. The validator does not make an untrusted configuration safe.
See [`CONFIG_DESIGN.md`](../../../CONFIG_DESIGN.md) for normative field details.
