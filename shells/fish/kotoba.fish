# Kotoba — fish integration.
# Place at ~/.config/fish/conf.d/kotoba.fish

if not type -q kotoba
    exit 0
end

set -gx KOTOBA_CACHE (set -q KOTOBA_CACHE; and echo $KOTOBA_CACHE; or echo "$HOME/.cache/kotoba/today")
set -gx KOTOBA_CACHE_TTL_SECONDS (set -q KOTOBA_CACHE_TTL_SECONDS; and echo $KOTOBA_CACHE_TTL_SECONDS; or echo 21600)

mkdir -p (dirname $KOTOBA_CACHE) 2>/dev/null

function _kotoba_refresh_today
    set -l now (date +%s)
    if test -f $KOTOBA_CACHE
        set -l mtime (stat -f %m $KOTOBA_CACHE 2>/dev/null; or stat -c %Y $KOTOBA_CACHE 2>/dev/null; or echo 0)
        if test (math "$now - $mtime") -lt $KOTOBA_CACHE_TTL_SECONDS
            return 0
        end
    end
    kotoba today > $KOTOBA_CACHE 2>/dev/null
end

function kotoba_today_prompt
    _kotoba_refresh_today
    if test -f $KOTOBA_CACHE
        cat $KOTOBA_CACHE
    end
end

_kotoba_refresh_today
set -gx KOTOBA_TODAY (cat $KOTOBA_CACHE 2>/dev/null)
