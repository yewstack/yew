// Standalone unit test for the yew-version remark logic.
// Run with: `node src/remark/yewVersionUrls.test.js` (no build / node_modules needed).
const assert = require('assert')
const { versionContextFromPath, applyTokens } = require('./yewVersionUrls.core')

const VERSIONS = ['0.23', '0.22', '0.21', '0.20']
let passed = 0
const check = (name, got, want) => {
    assert.deepStrictEqual(
        got,
        want,
        `${name}\n  got:  ${JSON.stringify(got)}\n  want: ${JSON.stringify(want)}`
    )
    passed++
}

const NEXT = {
    version: '0.23',
    isNext: true,
    display: '0.23',
    dependency: 'git = "https://github.com/yewstack/yew/"',
    api: 'https://yew-rs-api.web.app/next/yew/',
}
const V22 = {
    version: '0.22',
    isNext: false,
    display: '0.22',
    dependency: 'version = "0.22"',
    api: 'https://docs.rs/yew/0.22/yew/',
}

// --- version derivation from path (next / snapshot / i18n) ---
check(
    'next (docs/)',
    versionContextFromPath(
        '/repo/website/docs/getting-started/x.mdx',
        VERSIONS
    ),
    NEXT
)
check(
    'versioned 0.22',
    versionContextFromPath(
        '/repo/website/versioned_docs/version-0.22/tutorial/index.mdx',
        VERSIONS
    ),
    V22
)
check(
    'i18n next (current/)',
    versionContextFromPath(
        '/repo/website/i18n/ja/docusaurus-plugin-content-docs/current/x.mdx',
        VERSIONS
    ),
    NEXT
)
check(
    'i18n versioned 0.22',
    versionContextFromPath(
        '/repo/website/i18n/zh-Hans/docusaurus-plugin-content-docs/version-0.22/x.mdx',
        VERSIONS
    ),
    V22
)

// --- token substitution: next uses git dep + preview host; snapshots use version + docs.rs ---
const next = versionContextFromPath('/w/website/docs/x.mdx', VERSIONS)
const v22 = versionContextFromPath(
    '/w/website/versioned_docs/version-0.22/x.mdx',
    VERSIONS
)

check(
    'dependency (next -> git)',
    applyTokens('yew = { {{yew_dependency}}, features = ["csr"] }', next),
    'yew = { git = "https://github.com/yewstack/yew/", features = ["csr"] }'
)
check(
    'dependency (0.22 -> version)',
    applyTokens('yew = { {{yew_dependency}}, features = ["csr"] }', v22),
    'yew = { version = "0.22", features = ["csr"] }'
)
check(
    'api link (next -> preview host)',
    applyTokens('[use_future]({{yew_api}}suspense/fn.use_future.html)', next),
    '[use_future](https://yew-rs-api.web.app/next/yew/suspense/fn.use_future.html)'
)
check(
    'api link (0.22 -> docs.rs)',
    applyTokens('[use_future]({{yew_api}}suspense/fn.use_future.html)', v22),
    '[use_future](https://docs.rs/yew/0.22/yew/suspense/fn.use_future.html)'
)
check(
    'version token (next)',
    applyTokens('targets Yew {{yew_version}}', next),
    'targets Yew 0.23'
)
check(
    'version token (0.22)',
    applyTokens('targets Yew {{yew_version}}', v22),
    'targets Yew 0.22'
)
check('passthrough (no token)', applyTokens('plain text', next), 'plain text')

console.log(`ok — ${passed} assertions passed`)
