# Resource Protocol

## Identity And Versions

A resource identity is `<kind>:<namespace>/<name>`. A selection may append a
semantic-version requirement as `@<requirement>`. Identity does not contain a
URL, local path, display title, CLI version, or trust decision.

The supported kind IDs and workspace directories are:

| Kind | Directory |
| --- | --- |
| `knowledge` | `knowledge/` |
| `skill` | `skills/` |
| `workflow` | `workflows/` |
| `agent-profile` | `agent-profiles/` |
| `tool` | `tools/` |
| `prompt` | `prompts/` |
| `guide` | `guides/` |
| `template` | `templates/` |
| `collection` | `collections/` |

The namespace identifies a publisher boundary. It does not grant execution,
policy, workflow, or instruction precedence.

## Immutable Version Metadata

Each resource version declares:

- identity, semantic version, title, summary, authors, license, and source;
- trust class, publisher, review state, and optional review evidence;
- resource-schema and application compatibility requirements;
- required and optional dependency constraints;
- declared capabilities and executable-content indicators;
- collection members when the resource is a collection;
- every payload path, media type, length, SHA-256 digest, and immutable object
  location.

Workflow resources additionally declare a typed graph, referenced resources,
effects, required capabilities, artifacts, AI-bootstrap compatibility, and
decision-policy compatibility. This remains data. Manager never interprets or
executes a node.

Unknown required fields, unsupported schema versions, unsafe paths, malformed
requirements, duplicate identities, inconsistent graphs, and undeclared bytes
fail closed.

## Consumer State

The consumer-owned resource store is:

```text
.liknon/resources/
|-- manifest.json
|-- lock.json
|-- .metadata/
|-- knowledge/
|-- skills/
|-- workflows/
|-- agent-profiles/
|-- tools/
|-- prompts/
|-- guides/
|-- templates/
`-- collections/
```

`manifest.json` is desired state. It records explicit roots, accepted version
requirements, and whether each root comes from the Hub or an explicit local
directory. `lock.json` is the deterministic complete resolution. It records
exact versions, origins, dependency and collection edges, compatibility facts,
installation paths, payload inventory, catalog identity, and materializations.

Authenticated catalog bytes, their detached signature, and the separate trust
root are retained below `.metadata/` so offline verification does not depend on
a cache or network connection.

## Path And Byte Rules

Protocol paths are portable relative paths. Absolute paths, parent traversal,
empty components, platform-reserved names, control characters, unsafe
encodings, trailing dots or spaces, excessive lengths, symlink traversal, and
unsupported entry types are rejected.

Payload bytes are not normalized. Their declared length and lowercase SHA-256
digest cover exactly what is published and installed. Shared identical payload
bytes may point to one content-addressed Hub object.

The version 1 Hub transfers individual declared payload objects rather than
resource archives. Manager therefore performs no archive expansion; archive
ratio and nested-entry attacks are outside this transport shape instead of
being accepted and inspected after extraction.

## Compatibility And Activation

Compatibility is evidence, not authority. An incompatible resource is not
selected. A workflow that omits or does not satisfy AI-bootstrap or
decision-policy compatibility may be installed and inspected, but it is never
reported as activatable. Even a compatible workflow still requires a separate
host capability evaluation; Manager does not activate it.
