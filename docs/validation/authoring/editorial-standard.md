# Validation Knowledge Editorial Standard

## Scope

This local standard governs substantive guides and navigation indexes below
`docs/validation/knowledge/`. It profiles selected aspects of the publications
listed in [Editorial Bases](editorial-bases.md); it does not claim complete
conformance to any of them.

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

Introduce every fenced code block and identify its language. Label
placeholders and commands that require adaptation. Examples illustrate one
declared context; they never establish universal project policy.

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
metadata, navigation, links, sources, fixtures, embedding, and package checks
atomically. Do not leave redirects, aliases, forwarding files, duplicated
trees, or compatibility readers. Run the release check and complete the human
review rubric before marking a guide reviewed.
