// Pure, dependency-free helpers for the `yewVersionUrls` remark plugin.
//
// Kept separate from the remark wrapper so the interesting logic (deriving the
// documented version from a file path and expanding tokens) can be unit-tested
// with plain `node`, without pulling in the unified/unist toolchain.

// The `next` docs document the unreleased API, so they get the git dependency
// and the preview rustdoc host instead of a released version on docs.rs.
const GIT_DEPENDENCY = 'git = "https://github.com/yewstack/yew/"'
const PREVIEW_API_BASE = 'https://yew-rs-api.web.app/next/yew/'

/**
 * Determine what a docs source file documents, based on its path.
 *
 * - `website/docs/**`                              -> "next" (unreleased)
 * - `website/versioned_docs/version-X/**`          -> X
 * - `website/i18n/<l>/...content-docs/current/**`   -> "next"
 * - `website/i18n/<l>/...content-docs/version-X/**` -> X
 *
 * @param {string} filePath  absolute or relative path to the .md(x) source
 * @param {string[]} versions  contents of versions.json, newest first
 * @returns {{ version: string, isNext: boolean, display: string, dependency: string, api: string }}
 *   - `display`   : version string for prose (e.g. "0.23"; latest release on next)
 *   - `dependency`: Cargo.toml specifier (`git = "…"` on next, else `version = "X"`)
 *   - `api`       : rustdoc base URL for the `yew` crate, ending in `/yew/`
 *                   (preview host on next, `docs.rs/yew/X/yew/` on snapshots)
 */
function versionContextFromPath(filePath, versions) {
    const latest = versions[0]
    const norm = String(filePath).replace(/\\/g, '/')
    // Matches both `versioned_docs/version-0.23/…` and the i18n equivalent
    // `…/docusaurus-plugin-content-docs/version-0.23/…`.
    const m = norm.match(/(?:^|\/)version-(\d+\.\d+(?:\.\d+)?)(?:\/|$)/)
    if (m) {
        const v = m[1]
        return {
            version: v,
            isNext: false,
            display: v,
            dependency: `version = "${v}"`,
            api: `https://docs.rs/yew/${v}/yew/`,
        }
    }
    // `docs/…` (next) or the i18n `…/current/…` mirror.
    return {
        version: latest,
        isNext: true,
        display: latest,
        dependency: GIT_DEPENDENCY,
        api: PREVIEW_API_BASE,
    }
}

// Tokens authors write in the docs. Each resolves from the page's version ctx.
const TOKENS = {
    // Cargo.toml: `yew = { {{yew_dependency}}, features = ["csr"] }`
    '{{yew_dependency}}': (ctx) => ctx.dependency,
    // Links: `[use_future]({{yew_api}}suspense/fn.use_future.html)`
    '{{yew_api}}': (ctx) => ctx.api,
    // Prose: `targets Yew {{yew_version}}`
    '{{yew_version}}': (ctx) => ctx.display,
}

/** Replace every known yew token in `value` using `ctx`. */
function applyTokens(value, ctx) {
    if (typeof value !== 'string' || value.indexOf('{{yew') === -1) return value
    let out = value
    for (const token of Object.keys(TOKENS)) {
        if (out.indexOf(token) !== -1) {
            out = out.split(token).join(TOKENS[token](ctx))
        }
    }
    return out
}

module.exports = {
    versionContextFromPath,
    applyTokens,
    TOKENS,
    GIT_DEPENDENCY,
    PREVIEW_API_BASE,
}
