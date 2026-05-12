# OpenZed Git: Publish to GitHub TUI

## Overview

The `publish` command guides users through publishing a local project to GitHub. It handles git initialization, creating commits, and using `gh` CLI to create the repository.

## Flow Diagram

```
┌─────────────────────────────────────────────┐
│         OpenZed Git: Publish to GitHub       │
│         ═══════════════════════════         │
│                                             │
│  1. Git Check & Init                       │
│     ├─ Check if git repo exists            │
│     └─ Auto-init if missing                │
│                                             │
│  2. Branch Setup                           │
│     ├─ Ask default branch name (default: main)
│     └─ Rename if needed                     │
│                                             │
│  3. Initial Commit (if no commits)          │
│     ├─ Stage all files                     │
│     └─ Create initial commit               │
│                                             │
│  4. GitHub Authentication                   │
│     ├─ Check gh installed                  │
│     ├─ Check gh authenticated              │
│     └─ Ask repo details                    │
│                                             │
│  5. Create Repository                      │
│     ├─ Get repo name                       │
│     ├─ Get description                     │
│     ├─ Select visibility                  │
│     └─ Create via gh CLI                   │
│                                             │
│  6. Push & Done                            │
│     └─ Open in browser (optional)           │
└─────────────────────────────────────────────┘
```

## TUI Screens

### Step 1: Initial Check

```
────────────────────────────────────────────────────────
│          OpenZed Git: Publish to GitHub
────────────────────────────────────────────────────────

  Checking git... ✓
```

**Colors:**
- Separator title → `PINK`, bold
- Separator lines → `TEXT_DIM`
- Check mark → `GREEN` (`✓`)

---

### Step 2: Git Not Found - Auto Init

```
────────────────────────────────────────────────────────
│          OpenZed Git: Publish to GitHub
────────────────────────────────────────────────────────

  Checking git... ✗
  Initializing Git repository... ✓
```

**Colors:**
- Cross mark → `RED` (`✗`)
- Success mark → `GREEN` (`✓`)

---

### Step 3: Branch Setup

```
────────────────────────────────────────────────────────
│          OpenZed Git: Publish to GitHub
────────────────────────────────────────────────────────

  Git installed... ✓

Default branch: [main]:

No commits found. Stage all files and create initial commit? yes
Initial commit message: [initial commit]:
```

**Colors:**
- Labels → `TEXT_MUTED`
- Input text → `TEXT`
- Default values in brackets → `TEXT_DIM`

---

### Step 4: No Commits Prompt

```
────────────────────────────────────────────────────────
│          OpenZed Git: Publish to GitHub
────────────────────────────────────────────────────────

  Git installed... ✓
  GitHub CLI installed... ✓
  GitHub authenticated... ✓

No commits found. Stage all files and create initial commit? yes
Initial commit message: first
  ✓ Initial commit created
```

**Colors:**
- Question prompts → `TEXT`
- Success message → `GREEN`

---

### Step 5: Repository Details

```
────────────────────────────────────────────────────────
│          OpenZed Git: Publish to GitHub
────────────────────────────────────────────────────────

Repository name: [openzed-git]:
Description (optional) []:

Visibility:
  ▸ public
    private
```

**Colors:**
- Labels → `TEXT_MUTED`
- Input text → `TEXT`
- Selected item → `CYAN`
- Hover item → `TEXT_DIM`

---

### Step 6: Creating Repository

```
────────────────────────────────────────────────────────
│          OpenZed Git: Publish to GitHub
────────────────────────────────────────────────────────

  Git installed... ✓
  GitHub CLI installed... ✓
  GitHub authenticated... ✓

  Creating repository on GitHub... ✓
```

**Colors:**
- Progress indicators → `CYAN`

---

### Step 7: Success

```
────────────────────────────────────────────────────────
│          OpenZed Git: Publish to GitHub
────────────────────────────────────────────────────────

  Git installed... ✓
  GitHub CLI installed... ✓
  GitHub authenticated... ✓

  Creating repository on GitHub... ✓
  ✓ Pushed to GitHub

  ✓ Repository published successfully!

  GitHub: https://github.com/username/openzed-git

Open in browser? yes
```

**Colors:**
- All checkmarks → `GREEN`
- GitHub URL → `PURPLE`
- Success messages → `GREEN`

---

## Theme Colors Used

| Element | Color | Hex | Usage |
|---------|-------|-----|-------|
| `PINK` | Hot Pink | `#FF71E7` | Titles, headings |
| `GREEN` | Bright Green | `#9DFF65` | Success, ✓ checkmarks |
| `RED` | Coral Red | `#FF6767` | Errors, ✗ cross marks |
| `CYAN` | Teal Cyan | `#63D1A9` | Selected items, progress |
| `PURPLE` | Soft Purple | `#A277FF` | Links, URLs |
| `TEXT` | Off White | `#F8F8F2` | Main text content |
| `TEXT_MUTED` | Muted Blue | `#6272A4` | Labels, secondary text |
| `TEXT_DIM` | Dim Gray | `#6D6A7C` | Borders, separators, hints |

## ANSI Escape Codes Reference

```rust
// Titles and highlights
PINK = "\x1b[38;2;255;113;231m"     // #FF71E7

// Status indicators
GREEN = "\x1b[38;2;157;255;101m"   // #9DFF65
RED = "\x1b[38;2;255;103;103m"     // #FF6767
CYAN = "\x1b[38;2;99;209;169m"     // #63D1A9
PURPLE = "\x1b[38;2;162;119;255m"  // #A277FF

// Text
TEXT = "\x1b[38;2;248;248;242m"    // #F8F8F2
TEXT_MUTED = "\x1b[38;2;98;114;164m" // #6272A4
TEXT_DIM = "\x1b[38;2;109;106;124m"  // #6D6A7C

RESET = "\x1b[0m"
BOLD = "\x1b[1m"
```

## Component Styles

### Separator/Header

```rust
pub fn separator(title: &str) {
    let width = 54;
    let padding = (width - title.len()) / 2;
    println!();
    println!("{}─{}─", TEXT_DIM, "─".repeat(width));
    print!("{}│", SURFACE);
    print!("{}", " ".repeat(padding));
    aura_bold(title, PINK);  // Pink bold title
    println!();
    println!("{}─{}─", TEXT_DIM, "─".repeat(width));
    println!();
}
```

### Status Line

```rust
print!("  Git installed... ");
if git_installed {
    println!("{}", CHECK);  // GREEN ✓
} else {
    println!("{}", CROSS);  // RED ✗
}
```

### Input Prompt

```rust
let name: String = input_with_default("Repository name:", &default_name)?;
```

**Output:**
```
Repository name: [openzed-git]:
```

### Selection Menu

```rust
let visibility = dialoguer::Select::new()
    .with_prompt("Visibility")
    .items(&["public", "private"])
    .default(0)
    .interact()?;
```

**Output:**
```
Visibility:
  ▸ public
    private
```

## Implementation Files

| File | Purpose |
|------|---------|
| `src/commands/publish.rs` | Main publish flow command |
| `src/ui/aura.rs` | Theme colors and icons |
| `src/core/github.rs` | GitHub API interactions |
| `src/core/git.rs` | Git operations |

## Safety Features

1. **Never force push** - Uses standard `git push`
2. **Never auto-delete** - Only creates, never removes
3. **Confirmation prompts** - Asks before destructive actions
4. **Existing remote check** - Warns if remote already exists

## Error Handling

| Error | Message | Color |
|-------|---------|-------|
| Git not installed | `Git is not installed` | `RED` |
| Gh not installed | `GitHub CLI is not installed` | `RED` |
| Not authenticated | `GitHub CLI is not authenticated` | `RED` |
| Repo creation failed | `Failed to create repo: {error}` | `RED` |

**Example error display:**
```
────────────────────────────────────────────────────────
│          OpenZed Git: Publish to GitHub
────────────────────────────────────────────────────────

  Git installed... ✓
  GitHub CLI installed... ✓
  GitHub authenticated... ✗

  ✗ GitHub CLI is not authenticated.
    Run: gh auth login
```