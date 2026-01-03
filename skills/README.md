# lspcmd Skills

This directory contains [Agent Skills](https://agentskills.io) for lspcmd.

## Installation

### Prerequisites

Install lspcmd first:

```bash
pip install lspcmd
```

### Claude Code

```bash
# Add skills directory to Claude Code
cp -r skills/lspcmd ~/.claude/skills/
```

Or create a symlink:

```bash
ln -s /path/to/lspcmd/skills/lspcmd ~/.claude/skills/lspcmd
```

### OpenCode

```bash
# Add to OpenCode skills directory
cp -r skills/lspcmd ~/.opencode/skills/
```

Or create a symlink:

```bash
ln -s /path/to/lspcmd/skills/lspcmd ~/.opencode/skills/lspcmd
```

### Greger.el

Add the skills directory to your config:

```elisp
(add-to-list 'greger-skill-directories "/path/to/lspcmd/skills")
```

Or create a symlink:

```bash
ln -s /path/to/lspcmd/skills/lspcmd ~/.config/greger/skills/lspcmd
```

### Other Agents (Cursor, Windsurf, Aider, etc.)

For other agents that support the Agent Skills standard, copy the skill to the appropriate skills directory. See [agentskills.io](https://agentskills.io) for platform-specific instructions.

## Usage

Once installed, your agent will automatically use lspcmd when appropriate for:

- Finding symbol definitions
- Finding references to symbols
- Understanding call hierarchies
- Finding interface implementations
- Semantic refactoring

See [skills/lspcmd/SKILL.md](lspcmd/SKILL.md) for detailed documentation.

## License

MIT
