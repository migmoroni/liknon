# Direct Trust Root

This directory receives the public half of the persistent Validation manifest
signing identity as `public-key.pem` during external-readiness checkpoint AP-3.
Do not commit a private key, a temporary production placeholder, or a test key
at this path. Tests generate isolated disposable keys under temporary output.
