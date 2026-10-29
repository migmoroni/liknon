# Tool Direct Distribution Trust Material

This directory receives the production public key used to authenticate
`liknon-tool` direct-distribution metadata. The private key is never committed
and is available only to the protected `sign-tool-release` environment.

Provision `public-key.pem` during the B03 external-readiness procedure before
creating a native direct-distribution candidate.
