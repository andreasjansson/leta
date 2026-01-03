# leta Skills

This directory contains [Agent Skills](https://agentskills.io) for leta.

## Installation

### Prerequisites

Install leta first:

```bash
pip install leta
```

### Claude Code

```bash
# Add skills directory to Claude Code
cp -r skills/leta ~/.claude/skills/
```

Or create a symlink:

```bash
ln -s /path/to/leta/skills/leta ~/.claude/skills/leta
```

### OpenCode

```bash
# Add to OpenCode skills directory
cp -r skills/leta ~/.opencode/skills/
```

Or create a symlink:

```bash
ln -s /path/to/leta/skills/leta ~/.opencode/skills/leta
```

### Greger.el

Add the skills directory to your config:

```elisp
(add-to-list 'greger-skill-directories "/path/to/leta/skills")
```

Or create a symlink:

```bash
ln -s /path/to/leta/skills/leta ~/.config/greger/skills/leta
```

### Other Agents (Cursor, Windsurf, Aider, etc.)

For other agents that support the Agent Skills standard, copy the skill to the appropriate skills directory. See [agentskills.io](https://agentskills.io) for platform-specific instructions.

## Usage

Once installed, your agent will automatically use leta when appropriate for:

- Finding symbol definitions
- Finding references to symbols
- Understanding call hierarchies
- Finding interface implementations
- Semantic refactoring

See [skills/leta/SKILL.md](leta/SKILL.md) for detailed documentation.

## License

MIT
