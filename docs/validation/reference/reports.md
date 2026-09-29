# Report Reference

The version 4 `ValidationReport` is the canonical execution record. It includes
the selection, required tools, reached groups and suites, concrete check
commands and contexts, durations, statuses, exit codes, timeout and truncation
state, bounded captured streams, diagnostics, optional repository observations,
and an aggregate summary.

`pass` means every counted result in the selected graph passed. It does not
certify complete product correctness. `fail` identifies executed evidence that
failed; `blocked` identifies unavailable prerequisites; `skipped` identifies
work omitted because of dependency propagation or interruption.

Human and JSON views derive from the same completed typed report. JSON mode
writes one document to standard output without prose, progress rendering,
color, or terminal control bytes. Those exact bytes may be captured and hashed.

Repository evidence snapshots Git-visible state immediately before and after
execution. `before` and `after` preserve observations, while `introduced`,
`removed`, and `changed` distinguish mutations during the run from pre-existing
dirty state. An unavailable provider is a typed blocked gate. This mechanism
detects run-time mutations; it does not prove repository origin or historical
integrity.
