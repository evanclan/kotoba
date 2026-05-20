#!/usr/bin/env bash
# Kotoba — bash integration.
# Sourced from ~/.bashrc:
#     source /path/to/kotoba/shells/bash/kotoba.bash

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
    if mtime=$(stat -c %Y "$KOTOBA_CACHE" 2>/dev/null) \
       || mtime=$(stat -f %m "$KOTOBA_CACHE" 2>/dev/null); then
      if (( now - mtime < KOTOBA_CACHE_TTL_SECONDS )); then
        return 0
      fi
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

_kotoba_refresh_today
export KOTOBA_TODAY="$(cat "$KOTOBA_CACHE" 2>/dev/null)"
