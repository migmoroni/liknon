# Corpus Refoundation Review

This deterministic review artifact records the one-time Phase 2.1 inventory.
The canonical catalog remains the runtime source of metadata. Every row was
reviewed for reader question, profile, applicability, sources, version scope,
duplication, and canonical path on 2026-10-01.

| Stable ID | Profile and target path | Primary reader question | Applicability and sources | Version and duplication decision |
| --- | --- | --- | --- | --- |
| `concern.reproducible-release-evidence` | concern; `explanations/concerns/reproducible-release-evidence.md` | What evidence supports repeatable and complete source-to-artifact releases? | general, generated assets, release; `reproducible-builds.definition` | Ecosystem-neutral; removed tool-specific procedure and linked recipes. |
| `foundation.evidence-quality` | foundation; `explanations/foundations/evidence-quality.md` | When does an executed check meaningfully support a claim? | general; `google.flaky-tests`, `nist.ssdf-1.1` | Stable concepts; unified evidence, oracle, and residual-risk vocabulary. |
| `foundation.source-policy` | foundation; `explanations/foundations/source-policy.md` | How should local guidance use and cite external knowledge? | general; `json-schema.2020-12` | Stable provenance/applicability distinction; no duplicate register rules. |
| `foundation.test-types` | foundation; `explanations/foundations/test-types.md` | Which test boundary can observe the relevant failure? | general; `ieee.swebok-v4` | Local boundary terminology, explicitly non-universal. |
| `foundation.validation-model` | foundation; `explanations/foundations/validation-model.md` | Which evidence dimensions may be relevant? | general; `nist.ssdf-1.1` | Dimension inventory remains tool-neutral and non-prescriptive. |
| `foundation.validation-strategy` | foundation; `explanations/foundations/validation-strategy.md` | How should a contextual strategy be selected? | general; `nist.ssdf-1.1`, `w3c.wcag-2.2` | Preserves native external levels; routes procedures to recipes. |
| `language.javascript` | language; `explanations/languages/javascript.md` | Which language-level evidence applies to JavaScript? | JavaScript; `ecma.ecmascript-2025` | ECMAScript 2025 is intrinsic edition identity; Node.js moved to technology routing. |
| `language.rust` | language; `explanations/languages/rust.md` | Which language evidence supports a maintained Rust crate? | Cargo, Rust; `rust.cargo` | Edition/MSRV-oriented; Cargo, Clippy, and rustfmt commands remain references. |
| `language.typescript` | language; `explanations/languages/typescript.md` | What static evidence does TypeScript add? | JavaScript, TypeScript; `typescript.docs-6` | TypeScript 6 major line; separates emitted JavaScript and runtime behavior. |
| `pattern.evidence-chain` | pattern; `explanations/patterns/evidence-chain.md` | Why does a mechanism support a claim? | general; `nist.ssdf-1.1` | Context, forces, consequences, and failure modes made explicit. |
| `pattern.isolation-and-determinism` | pattern; `explanations/patterns/isolation-and-determinism.md` | How can results remain repeatable and attributable? | general; `google.flaky-tests`, `reproducible-builds.definition` | Ecosystem examples removed from the core pattern; links carry scope. |
| `pattern.oracles-and-failure-paths` | pattern; `explanations/patterns/oracles-and-failure-paths.md` | How should a check distinguish correctness and failure behavior? | general; `ieee.swebok-v4` | Independent-oracle forces and destructive-injection costs made explicit. |
| `recipe.javascript-typescript-package-gate` | recipe; `how-to/recipes/javascript-typescript-package-gate.md` | How can a Node.js TypeScript package be gated? | Git, JavaScript, Node.js, pnpm, release, TypeScript; `pnpm.cli-11`, `vitest.docs-4` | Declares Node.js 22 and pnpm 11 major assumptions; routes other lines to tool references. |
| `recipe.rust-cli-release-gate` | recipe; `how-to/recipes/rust-cli-release-gate.md` | How can a Cargo-managed Rust CLI be gated? | Cargo, CLI, Git, release, Rust; `rust.cargo` | Removed all workspace-validator-private assets and repository assumptions. |
| `standard.wcag-2.2` | standard; `reference/standards/wcag-22.md` | How should WCAG 2.2 conformance evidence be interpreted? | accessibility, Web; `w3c.wcag-2.2` | Exact 2.2 identity and A/AA/AAA meaning preserved. |
| `technology.git` | technology; `explanations/technologies/git.md` | Can Git-visible state detect unintended mutation? | Git; `git.status` | Stable porcelain concept; exact docs snapshot remains provenance. |
| `technology.nodejs` | technology; `explanations/technologies/nodejs.md` | Which runtime evidence supports Node.js compatibility? | JavaScript, Node.js, TypeScript; `nodejs.api-22` | Node.js 22 major scope; reviewed patch remains provenance. |
| `tool.cargo` | tool; `reference/tools/cargo.md` | Which Cargo command structure produces required evidence? | Cargo, release, Rust; `rust.cargo` | Active stable and declared MSRV lines; no patch identity. |
| `tool.clippy` | tool; `reference/tools/clippy.md` | What can a selected Clippy run establish? | Cargo, Rust; `rust.clippy` | Follows maintained Rust toolchain lines; exact snapshot is provenance. |
| `tool.git` | tool; `reference/tools/git.md` | How should Git status be invoked and interpreted? | Git; `git.status` | Maintained Git lines sharing porcelain v1; no patch identity. |
| `tool.pnpm` | tool; `reference/tools/pnpm.md` | What can pnpm dependency and script commands establish? | JavaScript, Node.js, pnpm, TypeScript; `pnpm.cli-11`, `pnpm.cli-12` | pnpm 11 and 12 are separate maintained lines with immutable provenance. |
| `tool.rustfmt` | tool; `reference/tools/rustfmt.md` | How can Rust formatting be checked without rewriting? | Cargo, Rust; `rust.rustfmt` | Follows maintained Rust toolchain lines; version sensitivity stated. |
| `tool.typescript-compiler` | tool; `reference/tools/typescript-compiler.md` | How can one TypeScript project be checked without emission? | JavaScript, Node.js, pnpm, TypeScript; `typescript.docs-6` | TypeScript 6 major line; exact patch remains provenance. |
| `tool.vitest` | tool; `reference/tools/vitest.md` | What can one configured Vitest run establish? | JavaScript, Node.js, pnpm, TypeScript; `vitest.docs-4` | Vitest 4 major line; exact patch remains provenance. |
| `tool.workspace-validator` | tool; `reference/tools/workspace-validator.md` | What does workspace-validator execute and report? | CLI, general; `workspace-validator.cli` | Installed crate version defines applicability; source commit remains provenance. |

No guide was split or merged because each stable ID retains one semantic
purpose. Navigation assets moved with their families and remain uncataloged.
No forwarding file, alias, or duplicate tree remains.
