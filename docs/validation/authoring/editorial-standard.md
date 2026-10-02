# Validation Knowledge Editorial Standard

## Scope

This local standard governs substantive guides and navigation indexes below
`docs/validation/knowledge/`. It profiles selected aspects of the publications
listed in [Editorial Bases](editorial-bases.md); it does not claim complete
conformance to any of them.

These conventions guide authors and human review. The release checker does not
judge prose completeness, heading order, citations, local links, or editorial
quality. It blocks only invalid machine-readable contracts, unsafe or missing
declared files, broken catalog references, unusable recipe configurations,
and stale generated source indexes. It also emits a deterministic digest of the
validated knowledge tree as release evidence.

## Families Profiles And Applicability

Choose one family from reader intent: explanation for understanding, how-to for
a bounded goal, and reference for precise lookup. Choose one concrete profile
from the catalog `kind`. Record subjects and workspace facts as applicability
facets. Never duplicate a guide merely to place its subject in another tree.

## Voice And Structure

Every substantive guide uses English, present tense, active voice, and direct,
reader-oriented prose. It has exactly one title-case H1, one primary reader
question, and the ordered H2 sequence in `editorial-profiles.json`. Do not skip
heading levels. H3 headings may only refine their parent H2.

Begin with purpose and applicability before commands or recommendations.
Distinguish observations, evidence, inference, recommendations, and external
normative requirements. Describe what evidence establishes and what remains
unknown. Keep depth proportional to the section's function; do not add filler
or enforce arbitrary word counts.

## Links Sources And Examples

Place a `[Source: source-id#location-id](authoritative URI)` citation next to
each substantive external claim. Use descriptive link text and link to a local
guide instead of repeating its content. Report missing coverage instead of
inventing guidance.

Local heading fragments use one deterministic slug function. Lowercase the
heading, retain Unicode letters and numbers plus `-` and `_`, remove other
punctuation, replace whitespace between retained characters with one `-`, and
trim leading or trailing `-`. Assign the unsuffixed slug to the first heading;
for every collision, append the first available `-1`, `-2`, and subsequent
integer suffix. Local links must name the exact resulting anchor. Percent
escapes must be complete and decode as UTF-8. Empty, malformed, missing, or
unresolvable local fragments should be corrected during review.

Introduce every fenced code block and identify its language. Label
placeholders and commands that require adaptation. Examples illustrate one
declared context; they never establish universal project policy.

A fenced block records its opening marker and length. Its authored closing
fence uses the same backtick or tilde marker at least as many times, starts
after no more than three spaces, and contains only spaces or tabs after the
markers. A parser end event at end of input does not count as an authored
closing fence.

## Effects And Safety

State material mutation, cost, privileges, network access, and external-service
effects. Prefer non-publishing examples. Scale recommendations to consequence
and uncertainty. Describe residual risk even when the proposed checks pass.

## Version And Source Scope

Guide prose covers stable behavior and materially distinct maintained major
lines. Exact tags, commits, editions, or dated living-documentation reviews
belong in `sources.json` as provenance. An exact reviewed snapshot does not
narrow public applicability. Keep exact versions in prose when the edition is
intrinsic, such as WCAG 2.2 or ECMAScript 2025, or when a patch changes the
described behavior.

Tool references list relevant major lines separately and tell readers to
confirm the active workspace version. Recipes declare capabilities and
major-line assumptions rather than an incidental workstation patch.

## Maintenance

Keep IDs stable while semantic purpose remains stable. Update paths, catalog
metadata, navigation, links, sources, embedding, and package checks atomically.
Draft catalog entries require only `id`, `kind`, `path`, and `status`; their
editorial metadata may remain absent while work is in progress. Complete every
catalog metadata field before changing an entry to `reviewed`.
Do not leave redirects, aliases, forwarding files, duplicated trees, or
compatibility readers. Run the technical release check and complete the human
review rubric before marking a guide reviewed.
