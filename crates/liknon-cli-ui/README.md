# Liknon CLI UI

`liknon-cli-ui` provides the product-neutral terminal experience shared by
Liknon command-line applications. It owns accessible palette and presentation
profiles, deterministic visual-option resolution, semantic style tokens, Clap
styling, compact and complete help presentation, and the canonical terminal
brand mark.

The crate intentionally does not define commands, reports, schemas, exit
codes, process execution, network behavior, or resource state. Applications
map their own concepts to the neutral semantic roles exposed here and retain
full ownership of their human and machine-readable contracts.

Human output is plain and ANSI-free by default. Explicit palettes enable ANSI
styling and override `NO_COLOR`; low-vision presentation may add non-color
typographic emphasis and expanded spacing. Machine-readable output must remain
ANSI-free, and callers can use the terminal policy helpers to reject visual
execution options rather than silently ignoring them.

```rust
use liknon_cli_ui::{PaletteProfile, PresentationProfile, SemanticRole, Theme};

let theme = Theme::resolve(PaletteProfile::Standard, PresentationProfile::LowVision);
let heading = theme.paint(SemanticRole::Heading, "Validation result");
assert_eq!(console::strip_ansi_codes(&heading), "Validation result");
```
