# Shell integrations

These integrations turn `kotoba today` into ambient practice — a word-of-the-day in your prompt, status bar, or `motd`.

> The official `kotoba shell init <shell>` subcommand is planned for v0.2 and will print the right snippet for each shell automatically. Until then, the snippets below are the same scripts that command will print.

## zsh

See [`shells/zsh/kotoba.zsh`](zsh/kotoba.zsh).

```sh
# In your ~/.zshrc:
source /path/to/kotoba/shells/zsh/kotoba.zsh
```

## fish

See [`shells/fish/kotoba.fish`](fish/kotoba.fish).

```fish
# In ~/.config/fish/conf.d/kotoba.fish:
cp shells/fish/kotoba.fish ~/.config/fish/conf.d/
```

## bash

See [`shells/bash/kotoba.bash`](bash/kotoba.bash).

```sh
# In your ~/.bashrc:
source /path/to/kotoba/shells/bash/kotoba.bash
```

## nushell

See [`shells/nushell/kotoba.nu`](nushell/kotoba.nu).

```nu
# In your nushell config:
source /path/to/kotoba/shells/nushell/kotoba.nu
```

## Best practices

- Cache the daily word so we don't shell out on every prompt repaint. The bundled scripts cache for 6 hours.
- Always degrade gracefully if `kotoba` isn't on `$PATH`.
- Keep the prompt fragment short. The full lookup is one keystroke away.

## Wanted

- Starship module
- Powerlevel10k segment
- tmux status-bar fragment
- powershell / pwsh integration
- elvish, xonsh, ion, others — open a PR
