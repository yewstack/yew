/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: 'Yew',
  tagline: 'Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.',
  url: 'https://yew.rs',
  baseUrl: '/',
  onBrokenLinks: 'warn',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/logo.png',
  organizationName: 'yewstack', // Usually your GitHub org/user name.
  projectName: 'yew', // Usually your repo name.
  themeConfig: {
    hideableSidebar: true,
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
          href: 'https://docs.rs/yew',
          position: 'right',
          label: 'API',
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
  },
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'ja', 'zh-CN', 'zh-TW'],
  },
  plugins: [
    'content-pages',
    [
      '@docusaurus/plugin-google-analytics',
      {
        trackingID: 'UA-141789564-1',
        anonymizeIP: true,
      },
    ],
    ['@docusaurus/theme-classic',
      {
        customCss: require.resolve('./src/css/custom.css'),
      }
    ],
    ['content-docs',
      {
        sidebarPath: require.resolve('./sidebars.js'),
        editUrl: 'https://github.com/yewstack/yew/blob/master/website/',
        routeBasePath: '/docs',
      }
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
      "@easyops-cn/docusaurus-search-local",
      {
        hashed: true,
        indexBlog: false
      },
    ],
  ],
};
