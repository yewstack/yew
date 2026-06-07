const { visit } = require('unist-util-visit')
const { versionContextFromPath, applyTokens } = require('./yewVersionUrls.core')

// versions.json lives at the website root (../../ relative to src/remark).
const versions = require('../../versions.json')

/**
 * Remark plugin: expand `{{yew_dependency}}`, `{{yew_api}}` and `{{yew_version}}`
 * placeholders to what the current docs page documents.
 *
 * `next` resolves to the git dependency and the preview rustdoc host (it documents
 * the unreleased API); a `version-X` snapshot resolves to `version = "X"` and
 * `docs.rs/yew/X`. Because each token is resolved from the page's own path at
 * build time, the snapshots created by `docusaurus docs:version` (and their
 * translations) keep resolving correctly with no manual edits — which is exactly
 * the maintenance burden this removes.
 *
 * Substitution is applied to text, inline code, fenced code blocks and link /
 * image URLs, so it works inside ```toml blocks (untouched by MDX interpolation)
 * as well as in prose links.
 */
function yewVersionUrls() {
    return (tree, file) => {
        const ctx = versionContextFromPath(
            file.path || (file.history && file.history[0]) || '',
            versions
        )

        visit(tree, (node) => {
            if (typeof node.value === 'string') {
                node.value = applyTokens(node.value, ctx) // text, code, inlineCode
            }
            if (typeof node.url === 'string') {
                node.url = applyTokens(node.url, ctx) // link, image, definition
            }
        })
    }
}

module.exports = yewVersionUrls
