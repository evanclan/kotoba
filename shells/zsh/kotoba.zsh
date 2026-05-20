#!/usr/bin/env zsh
# Kotoba — zsh integration.
#
# Sourced from your ~/.zshrc:
#     source /path/to/kotoba/shells/zsh/kotoba.zsh
#
# Provides:
#   - $KOTOBA_TODAY env var with the current word-of-the-day (cached for 6h).
#   - kotoba_today_prompt — a function you can include in PROMPT or RPROMPT.

if ! command -v kotoba >/dev/null 2>&1; then
  return 0
fi

KOTOBA_CACHE="${KOTOBA_CACHE:-${HOME}/.cache/kotoba/today}"
KOTOBA_CACHE_TTL_SECONDS="${KOTOBA_CACHE_TTL_SECONDS:-21600}"

mkdir -p "$(dirname "$KOTOBA_CACHE")" 2>/dev/null

_kotoba_refresh_today() {
  local now mtime
  now=$(date +%s)
  if [[ -f "$KOTOBA_CACHE" ]]; then
    mtime=$(stat -f %m "$KOTOBA_CACHE" 2>/dev/null || stat -c %Y "$KOTOBA_CACHE" 2>/dev/null || echo 0)
    if (( now - mtime < KOTOBA_CACHE_TTL_SECONDS )); then
      return 0
    fi
  fi
  kotoba today >"$KOTOBA_CACHE" 2>/dev/null || true
}

kotoba_today_prompt() {
  _kotoba_refresh_today
  if [[ -f "$KOTOBA_CACHE" ]]; then
    cat "$KOTOBA_CACHE"
  fi
}

# Make $KOTOBA_TODAY available to other scripts.
_kotoba_refresh_today
export KOTOBA_TODAY="$(cat "$KOTOBA_CACHE" 2>/dev/null)"
