# 🚨🚨🚨 MANDATORY: USE lspcmd NOT ripgrep 🚨🚨🚨

## ABSOLUTE RULE - NO EXCEPTIONS

**For ANY code symbol search, you MUST use `lspcmd`, NEVER `ripgrep`.**

This includes:
- Finding function definitions → `lspcmd show <name>` or `lspcmd grep <name> -k function`
- Finding class definitions → `lspcmd grep <name> -k class`
- Finding method bodies → `lspcmd show Class.method`
- Finding where something is defined → `lspcmd show <name>`
- Finding usages → `lspcmd refs <name>`

## SPECIFIC EXAMPLES YOU WILL ENCOUNTER

When you need to find a function like `is_excluded`:
```
❌ DO NOT: ripgrep "def is_excluded"
❌ DO NOT: ripgrep "is_excluded"  
❌ DO NOT: ripgrep "^def is_excluded"
✅ DO: lspcmd show is_excluded
```

When you need to find code in a file:
```
❌ DO NOT: ripgrep "pattern" file.py
✅ DO: lspcmd grep "pattern" file.py
```

## WHEN TO USE EACH TOOL

| Search Type | Tool | Example |
|------------|------|---------|
| Function/method | lspcmd | `lspcmd show is_excluded` |
| Class definition | lspcmd | `lspcmd grep Handler -k class` |
| Symbol usages | lspcmd | `lspcmd refs validate` |
| String literals | ripgrep | `ripgrep '"error message"'` |
| Comments/TODOs | ripgrep | `ripgrep "TODO"` |

## PRE-FLIGHT CHECK

Before EVERY search, ask:
> "Am I searching for a **function**, **class**, **method**, or **symbol**?"

If YES → **USE lspcmd** (show, grep, refs, calls)
If NO (string literals, comments) → OK to use ripgrep

**USING ripgrep FOR CODE SYMBOLS IS A MISTAKE. USE lspcmd.**
