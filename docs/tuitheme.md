# OpenZed Git TUI Theme

## Overview

OpenZed Git uses a modern dark theme with **White**, **Green**, and **Pink** as the primary accent colors. The theme is designed to be clean, readable, and visually appealing in terminal environments.

## Color Palette

### Text Colors

| Name | Hex (24-bit) | RGB | Usage |
|------|-------------|-----|-------|
| **TEXT** | `#FFFFFF` | (255, 255, 255) | Main text, values |
| **TEXT_MUTED** | `#B4B4B4` | (180, 180, 180) | Secondary labels, prompts |
| **TEXT_DIM** | `#787878` | (120, 120, 120) | Borders, separators, dimmed text |

### Accent Colors

| Name | Hex (24-bit) | RGB | Usage |
|------|-------------|-----|-------|
| **GREEN** | `#00FF87` | (0, 255, 135) | Success, checkmarks, staged files, commits |
| **PINK** | `#FF69B4` | (255, 105, 180) | Headings, arrows, branch icon, highlights |

### Status Colors

| Name | Hex (24-bit) | RGB | Usage |
|------|-------------|-----|-------|
| **SUCCESS** | `#00FF87` | (0, 255, 135) | Success messages, checkmarks |
| **WARNING** | `#FFFF64` | (255, 255, 100) | Warnings, modified files |
| **ERROR** | `#FF5050` | (255, 80, 80) | Errors, deleted files |
| **INFO** | `#64C8FF` | (100, 200, 255) | Info messages, blue accents |

### UI Elements

| Name | Hex (24-bit) | RGB | Usage |
|------|-------------|-----|-------|
| **SURFACE** | `#1E1E1E` | (30, 30, 30) | Box backgrounds |

## Icons

| Icon | Color | Symbol | Usage |
|------|-------|--------|-------|
| CHECK | Green | `✓` | Success, passed checks |
| CROSS | Red | `✗` | Errors, failed checks |
| WARN | Yellow | `⚠` | Warnings |
| ARROW | Pink | `›` | List item prefix |
| STAGED | Green | `●` | Staged files |
| MODIFIED | Yellow | `◎` | Modified files |
| DELETED | Red | `◼` | Deleted files |
| UNTRACKED | Gray | `○` | Untracked files |

## ANSI Escape Codes

The theme uses 24-bit true color ANSI escape codes:

```rust
// Format: \x1b[38;2;R;G;Bm (foreground) or \x1b[48;2;R;G;Bm (background)

// Example - Green text:
"\x1b[38;2;0;255;135m"

// Reset:
"\x1b[0m"

// Bold:
"\x1b[1m"
```

## Theme Constants

Located in `src/ui/aura.rs`:

```rust
pub mod aura {
    // Text colors
    pub const TEXT: &str = "\x1b[38;2;255;255;255m";        // White
    pub const TEXT_MUTED: &str = "\x1b[38;2;180;180;180m";  // Gray
    pub const TEXT_DIM: &str = "\x1b[38;2;120;120;120m";    // Dim gray

    // Accent colors
    pub const GREEN: &str = "\x1b[38;2;0;255;135m";         // Bright green
    pub const PINK: &str = "\x1b[38;2;255;105;180m";       // Hot pink

    // Status colors
    pub const SUCCESS: &str = "\x1b[38;2;0;255;135m";       // Bright green
    pub const WARNING: &str = "\x1b[38;2;255;255;100m";     // Yellow
    pub const ERROR: &str = "\x1b[38;2;255;80;80m";         // Red
    pub const INFO: &str = "\x1b[38;2;100;200;255m";        // Blue

    // UI Elements
    pub const SURFACE: &str = "\x1b[48;2;30;30;30m";        // Dark surface
}
```

## Usage Examples

### Separator with Title

```rust
pub fn separator(title: &str) {
    let width = 50;
    let padding = (width - title.len()) / 2;
    println!();
    println!("{}─{}─", aura::TEXT_DIM, "─".repeat(width));
    print!("{}│", aura::SURFACE);
    print!("{}", " ".repeat(padding));
    aura_bold(title, aura::PINK);  // Pink bold title
    println!();
    println!("{}─{}─", aura::TEXT_DIM, "─".repeat(width));
    println!();
}
```

**Output:**
```
────────────────────────────────────────────────────
│              OpenZed Git: Doctor
────────────────────────────────────────────────────
```

### Status Check with Icon

```rust
print!("  Git installed... ");
if git_installed {
    println!("{}", CHECK);  // Green checkmark
} else {
    println!("{}", CROSS);  // Red cross
}
```

**Output:**
```
  Git installed... ✓
```

### Key-Value Item

```rust
pub fn item<K: Display, V: Display>(key: K, value: V) {
    print!("  {} ", aura::ARROW);                    // Pink arrow
    aura_text(&format!("{}:", key), aura::TEXT_MUTED);  // Gray label
    aura_text(&value.to_string(), aura::TEXT);          // White value
    println!();
}
```

**Output:**
```
  › Status: main...origin/main
```

## Theme File Structure

```
src/ui/
├── mod.rs      # Spinner, ProgressBar animations
├── aura.rs     # Theme colors, icons, helper functions
├── banner.rs   # Startup banner
└── prompts.rs  # User input prompts
```

## Loading Animations

### Spinner

```rust
pub struct Spinner {
    message: String,
    done_message: String,
    frames: Vec<String>,
}
```

Frames: `⠋⠙⠹⠸⠼⠴⠦⠧`

### Progress Bar

```rust
pub struct ProgressBar {
    total: usize,
    current: usize,
    width: usize,
}
```

Rendered with `█` (filled) and `░` (empty) blocks.

## Customization

To change the theme, modify the constants in `src/ui/aura.rs`:

1. **Change accent colors** - Update `GREEN` and `PINK`
2. **Change status colors** - Update `SUCCESS`, `WARNING`, `ERROR`, `INFO`
3. **Change icons** - Update the icon constants with new symbols

## Dark/Light Mode Support

Currently, only dark mode is implemented. For light mode support, you would need to:

1. Create a separate `light` module
2. Add a runtime theme switcher
3. Update colors to be readable on light backgrounds
