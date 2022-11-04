const { API_BUTTON } = require('./src/constants')

const editUrl = 'https://github.com/yewstack/yew/blob/master/website/'

/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
    title: 'Yew',
    tagline:
        'A framework for creating reliable and efficient web applications.',
    url: 'https://yew.rs',
    baseUrl: '/',
    onBrokenLinks: 'throw',
    onBrokenMarkdownLinks: 'warn',
    favicon: 'img/logo.png',
    organizationName: 'yewstack', // Usually your GitHub org/user name.
    projectName: 'yew', // Usually your repo name.
    themeConfig: {
        docs: {
            sidebar: {
                hideable: true,
            },
        },
        navbar: {
            title: 'Yew',
            logo: {
                alt: 'Yew Logo',
                src: 'img/logo.png',
            },
            items: [
                {
                    type: 'docsVersionDropdown',
                    position: 'left',
                },
                {
                    type: 'localeDropdown',
                    position: 'left',
                },
                {
                    type: 'doc',
                    position: 'left',
                    docId: 'getting-started/introduction',
                    label: 'Docs',
                },
                {
                    type: 'doc',
                    position: 'left',
                    docId: 'tutorial/index',
                    label: 'Tutorial',
                },
                {
                    docsPluginId: 'community',
                    type: 'doc',
                    position: 'right',
                    docId: 'awesome',
                    label: 'Community',
                },
                {
                    position: 'right',
                    to: 'blog',
                    label: 'Blog',
                },
                {
                    href: 'https://play.yew.rs/',
                    position: 'right',
                    label: 'Playground',
                },
                {
                    href: 'https://docs.rs/yew',
                    position: 'right',
                    label: API_BUTTON,
                },
                {
                    href: 'https://github.com/yewstack/yew',
                    label: 'GitHub',
                    position: 'right',
                },
            ],
        },
        footer: {
            style: 'dark',
            links: [
                {
                    title: 'Support',
                    items: [
                        {
                            label: 'Sponsor Project',
                            href: 'https://opencollective.com/yew',
                        },
                    ],
                },
                {
                    title: 'Participate',
                    items: [
                        {
                            label: 'GitHub',
                            href: 'https://github.com/yewstack/yew',
                        },
                        {
                            label: 'Discord',
                            href: 'https://discord.gg/VQck8X4',
                        },
                        {
                            label: 'Twitter',
                            href: 'https://twitter.com/yewstack',
                        },
                    ],
                },
                {
                    title: 'More',
                    items: [
                        {
                            label: 'Yew Awesome',
                            href: 'https://github.com/jetli/awesome-yew',
                        },
                    ],
                },
            ],
        },
        prism: {
            additionalLanguages: ['rust', 'toml'],
        },
        googleAnalytics: {
            trackingID: 'UA-141789564-1',
            anonymizeIP: true,
        },
    },
    i18n: {
        defaultLocale: 'en',
        locales: ['en', 'ja', 'zh-Hans', 'zh-Hant'],
    },
    plugins: [
        'content-pages',
        'docusaurus-plugin-sass',
        [
            '@docusaurus/theme-classic',
            {
                customCss: require.resolve('./src/css/custom.css'),
            },
        ],
        [
            '@docusaurus/plugin-content-docs',
            {
                path: 'docs',
                sidebarPath: require.resolve('./sidebars/docs.js'),
                editUrl,
                routeBasePath: '/docs',
            },
        ],
        [
            '@docusaurus/plugin-content-docs',
            {
                id: 'community',
                path: 'community',
                sidebarPath: require.resolve('./sidebars/community.js'),
                routeBasePath: '/community',
                editUrl,
            },
        ],
        [
            '@docusaurus/plugin-content-blog',
            {
                path: 'blog',
                blogTitle: 'Yew Blog',
                editUrl,
            },
        ],
        [
            'client-redirects',
            {
                redirects: [
                    // this handles the redirect from `/next` -> to the (current) first item in the docs sidebar
                    // note: if the first item is changed, it should be reflected here
                    {
                        to: '/docs/next/getting-started/introduction', // string
                        from: ['/docs/next'], // string | string[]
                    },
                ],
            },
        ],
        [
            '@easyops-cn/docusaurus-search-local',
            {
                hashed: true,
                indexBlog: false,
            },
        ],
    ],
}
