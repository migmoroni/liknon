# Liknon CLI Suite Distribution

This directory is the canonical native-distribution contract for the complete
Liknon CLI suite. Each supported target produces one authenticated archive
containing exactly these distinct executables:

```text
liknon
liknon-validator
liknon-manager
liknon-tool
```

`product.json` owns the target matrix, release identity, protected publication
environments, archive formats, and platform-authenticity requirements. Shared
assembly and verification code lives under `distribution/cli/common/`.

The suite is versioned and promoted atomically. Capability CLIs do not publish
standalone native archives or stable channels.
