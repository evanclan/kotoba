# Kotoba — nushell integration.
# Source from your nushell config:
#     source /path/to/kotoba/shells/nushell/kotoba.nu

if (which kotoba | is-empty) {
    # `kotoba` is not on PATH; do nothing.
} else {
    let cache = ($env.KOTOBA_CACHE? | default ($env.HOME | path join ".cache" "kotoba" "today"))
    mkdir ($cache | path dirname)

    def _kotoba_refresh_today [] {
        let cache = ($env.KOTOBA_CACHE? | default ($env.HOME | path join ".cache" "kotoba" "today"))
        let ttl = ($env.KOTOBA_CACHE_TTL_SECONDS? | default 21600 | into int)
        if ($cache | path exists) {
            let mtime = (ls $cache | get 0.modified | into int) / 1_000_000_000
            let now = (date now | into int) / 1_000_000_000
            if (($now - $mtime) < $ttl) {
                return
            }
        }
        do { kotoba today | save -f $cache } | ignore
    }

    def kotoba_today_prompt [] {
        _kotoba_refresh_today
        let cache = ($env.KOTOBA_CACHE? | default ($env.HOME | path join ".cache" "kotoba" "today"))
        if ($cache | path exists) { open --raw $cache } else { "" }
    }

    _kotoba_refresh_today
    let cache = ($env.KOTOBA_CACHE? | default ($env.HOME | path join ".cache" "kotoba" "today"))
    $env.KOTOBA_TODAY = (if ($cache | path exists) { open --raw $cache } else { "" })
}
