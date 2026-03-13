use stylist::yew::styled_component;
use yew::prelude::*;
use yew_site_lib::Layout;

const VERSIONS: &[(&str, &str)] = &[
    ("All versions", ""),
    ("Next", "next"),
    ("0.23", "0.23"),
    ("0.22", "0.22"),
    ("0.21", "0.21"),
    ("0.20", "0.20"),
];

#[styled_component]
pub fn Page() -> Html {
    use_effect_with((), |_| {
        init_search();
        || {}
    });

    let style = css!(
        r#"
        max-width: 800px;
        margin: 0 auto;
        padding: 2rem 1rem;

        .search-header {
            display: flex;
            gap: 0.75rem;
            align-items: center;
            margin-bottom: 2rem;
        }

        .search-input {
            flex: 1;
            padding: 0.75rem 1rem;
            font-size: 1.125rem;
            border: 2px solid var(--color-border);
            border-radius: 6px;
            background: var(--color-bg);
            color: var(--color-text);
            font-family: inherit;
            outline: none;
            transition: border-color 0.2s;
        }

        .search-input:focus {
            border-color: var(--color-primary);
        }

        .search-version {
            padding: 0.75rem 0.75rem;
            font-size: 0.9375rem;
            border: 2px solid var(--color-border);
            border-radius: 6px;
            background: var(--color-bg);
            color: var(--color-text);
            font-family: inherit;
            cursor: pointer;
        }

        .search-results {
            min-height: 200px;
        }

        .search-result {
            margin-bottom: 1.5rem;
            padding-bottom: 1.5rem;
            border-bottom: 1px solid var(--color-border);
        }

        .search-result:last-child {
            border-bottom: none;
        }

        .search-result h3 {
            margin: 0 0 0.25rem;
            font-size: 1.125rem;
        }

        .search-result h3 a {
            color: var(--color-primary);
            text-decoration: none;
        }

        .search-result h3 a:hover {
            text-decoration: underline;
        }

        .search-result-breadcrumb {
            font-size: 0.8125rem;
            color: var(--color-text-secondary);
            margin-bottom: 0.25rem;
        }

        .search-result-content {
            margin: 0;
            color: var(--color-text-secondary);
            font-size: 0.9375rem;
            line-height: 1.5;
        }

        .search-result-content mark {
            background: rgba(var(--color-primary-rgb, 37, 194, 160), 0.2);
            color: inherit;
            padding: 0 0.125rem;
            border-radius: 2px;
        }

        .search-empty {
            color: var(--color-text-secondary);
            font-size: 1.125rem;
            text-align: center;
            padding: 3rem 0;
        }

        .search-loading {
            color: var(--color-text-secondary);
            text-align: center;
            padding: 2rem 0;
        }

        .search-footer {
            text-align: center;
            padding: 1rem 0;
            font-size: 0.8125rem;
            color: var(--color-text-secondary);
        }

        .search-footer a {
            color: var(--color-primary);
        }
        "#
    );

    html! {
        <Layout title="Search" active_nav="">
            <div class={style}>
                <div class="search-header">
                    <input
                        class="search-input"
                        id="search-input"
                        type="search"
                        placeholder="Search docs..."
                        autocomplete="off"
                        autofocus=true
                    />
                    <select class="search-version" id="search-version">
                        { for VERSIONS.iter().map(|(label, value)| html! {
                            <option value={*value}>{label}</option>
                        })}
                    </select>
                </div>
                <div class="search-results" id="search-results" />
                <div class="search-footer">
                    {"Search by "}
                    <a href="https://www.algolia.com/developers/" target="_blank" rel="noopener noreferrer">
                        {"Algolia"}
                    </a>
                </div>
            </div>
        </Layout>
    }
}

#[cfg(feature = "csr")]
pub fn init_search() {
    let _ = js_sys::eval(SEARCH_JS);
}

#[cfg(not(feature = "csr"))]
pub fn init_search() {}

#[cfg(feature = "csr")]
const SEARCH_JS: &str = r#"
(function() {
    var ALGOLIA_APP_ID = 'F8S2ICRD2T';
    var ALGOLIA_API_KEY = '6a9cd0bf0d86b8d643b5e609e7755248';
    var ALGOLIA_INDEX = 'yew-rs';

    var input = document.getElementById('search-input');
    var results = document.getElementById('search-results');
    var versionSelect = document.getElementById('search-version');
    if (!input || !results) return;

    var params = new URLSearchParams(location.search);
    var q = params.get('q') || '';
    if (q) input.value = q;

    var debounceTimer;

    function loadAlgolia(cb) {
        if (window.algoliasearch) { cb(); return; }
        var s = document.createElement('script');
        s.src = 'https://cdn.jsdelivr.net/npm/algoliasearch@4/dist/algoliasearch-lite.umd.js';
        s.onload = cb;
        document.head.appendChild(s);
    }

    function escapeHtml(str) {
        var d = document.createElement('div');
        d.textContent = str;
        return d.innerHTML;
    }

    function doSearch() {
        var query = input.value.trim();
        if (!query) {
            results.innerHTML = '';
            history.replaceState(null, '', location.pathname);
            return;
        }
        history.replaceState(null, '', '?q=' + encodeURIComponent(query));

        loadAlgolia(function() {
            var client = algoliasearch(ALGOLIA_APP_ID, ALGOLIA_API_KEY);
            var index = client.initIndex(ALGOLIA_INDEX);

            var facetFilters = ['language:en'];
            var version = versionSelect ? versionSelect.value : '';
            if (version) facetFilters.push('version:' + version);

            index.search(query, {
                hitsPerPage: 20,
                facetFilters: facetFilters,
                highlightPreTag: '<mark>',
                highlightPostTag: '</mark>'
            }).then(function(res) {
                if (res.hits.length === 0) {
                    results.innerHTML = '<p class="search-empty">No results found for "' + escapeHtml(query) + '"</p>';
                    return;
                }
                var html = '';
                res.hits.forEach(function(hit) {
                    var h = hit.hierarchy || {};
                    var title = h.lvl1 || h.lvl2 || h.lvl0 || 'Untitled';
                    var breadcrumb = [h.lvl0, h.lvl1, h.lvl2].filter(Boolean);
                    var url = hit.url || '#';
                    url = url.replace('https://yew.rs', '');

                    var hl = hit._highlightResult || {};
                    var hlH = hl.hierarchy || {};
                    var hlTitle = (hlH.lvl1 && hlH.lvl1.value) || (hlH.lvl2 && hlH.lvl2.value) || (hlH.lvl0 && hlH.lvl0.value) || escapeHtml(title);
                    var hlContent = (hl.content && hl.content.value) || '';

                    html += '<div class="search-result">';
                    if (breadcrumb.length > 1) {
                        html += '<div class="search-result-breadcrumb">' + breadcrumb.map(escapeHtml).join(' > ') + '</div>';
                    }
                    html += '<h3><a href="' + escapeHtml(url) + '">' + hlTitle + '</a></h3>';
                    if (hlContent) html += '<p class="search-result-content">' + hlContent + '</p>';
                    html += '</div>';
                });
                results.innerHTML = html;
            });
        });
    }

    input.addEventListener('input', function() {
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(doSearch, 300);
    });
    if (versionSelect) versionSelect.addEventListener('change', doSearch);
    if (q) doSearch();
    input.focus();
})();
"#;
