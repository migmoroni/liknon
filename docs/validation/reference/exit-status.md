# Exit Status Reference

| Code | Meaning |
| ---: | --- |
| `0` | The request completed with a positive result: all counted validation evidence passed, or every initialization resource was created/reused |
| `1` | The request completed with negative evidence: validation failed, or initialization found a conflict/partial result |
| `2` | Validation produced no failure but could not complete selected evidence because at least one result was blocked or skipped |
| `3` | The request was rejected before configured execution because CLI usage, configuration, candidate input, or a provisioning precondition was invalid |
| `4` | The validator could not construct, execute, provision, or render the requested result because of an internal operational failure |
| `130` | An interrupt signal was observed during validation; this code takes precedence over the report aggregate |

Each code has the same meaning for human and JSON presentation. Interpret the
typed result as the primary machine contract and use the exit code for process
composition. Initialization never returns `2` or `130` because it starts no
configured process.
