# Theme System

OpenZed Git supports multiple themes. The default theme is **Aura Dark**.

## Available Themes

| Theme | Description |
|-------|-------------|
| `aura-dark` | Aura Dark - Purple/pink accent, dark background (**default**) |
| `minimal-dark` | Minimal Dark - Clean, simple dark theme |
| `zed-dark` | Zed Dark - Match Zed editor's dark theme |

## Theme Commands

```bash
openzed-git theme              # Open theme menu
openzed-git theme list         # List available themes
openzed-git theme preview     # Preview a theme
openzed-git theme set <name>   # Set active theme
```

## Setting a Theme

Themes are set in the global config file at `~/.config/openzed-git/config.toml`:

```toml
theme = "aura-dark"
```

## Theme Preview

Use `openzed-git theme preview` to see sample UI elements in each theme:
- Success/warning/error messages
- Links
- Selected menu items
- File names and refs
- Header titles

---

# Aura Dark Theme

OpenZed Git uses a carefully crafted dark theme called **Aura Dark** for its terminal user interface.

## Design Principles

- **Readable**: High contrast text on dark backgrounds
- **Consistent**: Same colors used for same purposes throughout
- **Beautiful**: Carefully chosen accent colors
- **Accessible**: 24-bit true color support

## Color Palette

### Background Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| `BG` | `#1C1B22` | (28, 27, 34) | Main background |
| `SURFACE` | `#21222C` | (33, 34, 44) | Panel/card backgrounds |

### Text Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| `TEXT` | `#F8F8F2` | (248, 248, 242) | Main text, values |
| `TEXT_MUTED` | `#6272A4` | (98, 114, 164) | Labels, secondary text |
| `TEXT_DIM` | `#6D6A7C` | (109, 106, 124) | Borders, separators, hints |

### Accent Colors

| Name | Hex | RGB | Usage |
|------|-----|-----|-------|
| `PURPLE` | `#A277FF` | (162, 119, 255) | Links, URLs, branch refs |
| `PINK` | `#FF71E7` | (255, 113, 231) | Titles, headings, arrows |
| `CYAN` | `#63D1A9` | (99, 209, 169) | Selected items, progress, graph symbols |
| `GREEN` | `#9DFF65` | (157, 255, 101) | Success, checkmarks, staged files |
| `YELLOW` | `#FFE46A` | (255, 228, 106) | Warnings, modified files |
| `RED` | `#FF6767` | (255, 103, 103) | Errors, deleted files |
| `WHITE` | `#FFFFFF` | (255, 255, 255) | Emphasis |

## Color Usage Guide

Use colors consistently for specific purposes:

| Purpose | Color |
|---------|-------|
| Titles/Headers | `PINK` + bold |
| Main text content | `TEXT` |
| Labels and prompts | `TEXT_MUTED` |
| Separators and borders | `TEXT_DIM` |
| Links and URLs | `PURPLE` |
| Selected menu items | `CYAN` |
| Success messages | `GREEN` |
| Warning messages | `YELLOW` |
| Error messages | `RED` |
| Progress indicators | `CYAN` |
| Graph symbols | `CYAN` |
| Commit hash | `CYAN` |
| Dates | `TEXT_MUTED` |
| Branch refs | `PURPLE` or `YELLOW` |
| Arrows | `PINK` |

## Rust Constants

```rust
// Background colors
pub const BG: &str = "\x1b[48;2;28;27;34m";
pub const SURFACE: &str = "\x1b[48;2;33;34;44m";

// Text colors
pub const TEXT: &str = "\x1b[38;2;248;248;242m";
pub const TEXT_MUTED: &str = "\x1b[38;2;98;114;164m";
pub const TEXT_DIM: &str = "\x1b[38;2;109;106;124m";

// Accent colors
pub const PURPLE: &str = "\x1b[38;2;162;119;255m";
pub const PINK: &str = "\x1b[38;2;255;113;231m";
pub const CYAN: &str = "\x1b[38;2;99;209;169m";
pub const GREEN: &str = "\x1b[38;2;157;255;101m";
pub const YELLOW: &str = "\x1b[38;2;255;228;106m";
pub const RED: &str = "\x1b[38;2;255;103;103m";
pub const WHITE: &str = "\x1b[38;2;255;255;255m";

// Reset and style
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
```

## Icons

| Icon | Color | Symbol | Usage |
|------|-------|--------|-------|
| `CHECK` | GREEN | `✓` | Success, passed checks |
| `CROSS` | RED | `✗` | Errors, failed checks |
| `WARN` | YELLOW | `⚠` | Warnings |
| `ARROW` | PINK | `›` | List item prefix |
| `SELECTED` | CYAN | `▸` | Selected menu item |
| `BULLET` | CYAN | `●` | Bullet points |
| `STAGED` | GREEN | `●` | Staged files |
| `MODIFIED` | YELLOW | `◎` | Modified files |
| `DELETED` | RED | `◼` | Deleted files |
| `UNTRACKED` | TEXT_MUTED | `○` | Untracked files |

## Graph Symbols

| Symbol | Color | Usage |
|--------|-------|-------|
| `●` | CYAN | Current commit |
| `│` | CYAN | Graph line |
| `⎇` | PURPLE | Branch |
| `→` | PINK | Arrow/pointer |

## Example Usage

### Separator with Title

```rust
separator("OpenZed Git: Publish to GitHub");

// Output:
// ────────────────────────────────────────────────────
// │        OpenZed Git: Publish to GitHub
// ────────────────────────────────────────────────────
```

### Status Check

```rust
print!("  Checking Git... ");
if git_ok {
    success("Git installed");
} else {
    error("Git not found");
}

// Output:
//   Checking Git... ✓ Git installed
//   Checking Git... ✗ Git not found
```

### Labeled Value

```rust
labeled_value("Repository", "openzed-git");
labeled_value("Branch", "main");
labeled_value("Remote", "origin");

// Output:
//   Repository openzed-git
//   Branch main
//   Remote origin
```

## Theme File

All theme constants are defined in:

```
crates/openzed-git/src/ui/aura.rs
```

## Customization

To customize the theme:

1. Edit constants in `src/ui/aura.rs`
2. Ensure 24-bit color support in your terminal
3. Test on multiple terminals for consistency

## Terminal Support

Aura Dark requires a terminal with 24-bit color support:

- iTerm2 (macOS)
- Konsole (Linux)
- Windows Terminal
- Alacritty
- Kitty
