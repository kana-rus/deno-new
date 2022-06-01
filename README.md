# Usage (plan)

```
deno-setup [options] dirname
```
This setups deno project directory with

- deno.json (with some default contents)
- import_map.json (empty)
- src/main.ts (empty)
- editor's config directory or file (default : .vscode/settings.json with settings for enabling deno extention)

<br>

# Options (plan)
- --editor, -e ... explicitly specifying the edotor (default: code)
- 