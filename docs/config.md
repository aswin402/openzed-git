# Configuration

OpenZed Git supports configuration files for customizing behavior.

## Global Config

Location: `~/.config/openzed-git/config.toml`

### Example Config

```toml
# OpenZed Git Global Configuration

default_branch = "main"
default_visibility = "public"
auto_open_browser = true
theme = "aura-dark"
commit_style = "conventional"
default_pr_base = "main"
remote = "origin"
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `default_branch` | string | `"main"` | Default branch name for new repos |
| `default_visibility` | string | `"public"` | Default repo visibility |
| `auto_open_browser` | boolean | `true` | Auto-open browser after publish |
| `theme` | string | `"aura-dark"` | TUI theme |
| `commit_style` | string | `"conventional"` | Commit message style |
| `default_pr_base` | string | `"main"` | Default base branch for PRs |
| `remote` | string | `"origin"` | Default remote name |
| `github_owner` | string | - | GitHub owner for PR creation |

## Project Config

Location: `.openzed-git.toml` in project root

### Example Project Config

```toml
# OpenZed Git Project Configuration

project_name = "my-project"
default_branch = "main"
remote = "origin"
github_owner = "aswinvishal402"
auto_open_browser = false
```

### Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `default_branch` | string | global | Override default branch |
| `remote` | string | `"origin"` | Default remote name |
| `github_owner` | string | - | GitHub owner for PR creation |
| `auto_open_browser` | boolean | true | Override browser auto-open |

## Config Precedence

1. Project config (`.openzed-git.toml`) takes precedence
2. Global config (`~/.config/openzed-git/config.toml`) is fallback
3. CLI defaults are used if neither config exists

**Note:** Missing or invalid config files are silently ignored. Safe defaults are always used.

## Commands

Manage config via CLI:

```bash
openzed-git config          # Open config menu
openzed-git config show     # Show current merged config
openzed-git config init     # Create global config
```

## Commit Styles

### conventional

Generates conventional commit messages:

```
<type>(<scope>): <subject>

<type> = feat|fix|docs|style|refactor|test|chore
<scope> = optional module name
<subject> = brief description
```

### simple

Simple single-line commit messages without type prefix.
