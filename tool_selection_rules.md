# 🚨🚨🚨 MANDATORY: USE leta NOT ripgrep 🚨🚨🚨

## ABSOLUTE RULE - NO EXCEPTIONS

**For ANY code symbol search, you MUST use `leta`, NEVER `ripgrep`.**

This includes:
- Finding function definitions → `leta show <name>` or `leta grep <name> -k function`
- Finding class definitions → `leta grep <name> -k class`
- Finding method bodies → `leta show Class.method`
- Finding where something is defined → `leta show <name>`
- Finding usages → `leta refs <name>`

## SPECIFIC EXAMPLES YOU WILL ENCOUNTER

When you need to find a function like `is_excluded`:
```
❌ DO NOT: ripgrep "def is_excluded"
❌ DO NOT: ripgrep "is_excluded"  
❌ DO NOT: ripgrep "^def is_excluded"
✅ DO: leta show is_excluded
```

When you need to find code in a file:
```
❌ DO NOT: ripgrep "pattern" file.py
✅ DO: leta grep "pattern" file.py
```

## WHEN TO USE EACH TOOL

| Search Type | Tool | Example |
|------------|------|---------|
| Function/method | leta | `leta show is_excluded` |
| Class definition | leta | `leta grep Handler -k class` |
| Symbol usages | leta | `leta refs validate` |
| String literals | ripgrep | `ripgrep '"error message"'` |
| Comments/TODOs | ripgrep | `ripgrep "TODO"` |

## PRE-FLIGHT CHECK

Before EVERY search, ask:
> "Am I searching for a **function**, **class**, **method**, or **symbol**?"

If YES → **USE leta** (show, grep, refs, calls)
If NO (string literals, comments) → OK to use ripgrep

**USING ripgrep FOR CODE SYMBOLS IS A MISTAKE. USE leta.**
