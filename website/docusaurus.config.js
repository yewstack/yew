/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: 'Yew',
  tagline: 'Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.',
  url: 'https://yew.rs',
  baseUrl: '/',
  onBrokenLinks: 'warn',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/favicon.png',
  organizationName: 'yewstack', // Usually your GitHub org/user name.
  projectName: 'yew', // Usually your repo name.
  themeConfig: {
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
          docId: 'index',
          position: 'left',
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
              label: 'Fund Issues',
              href: 'https://issuehunt.io/r/yewstack/yew',
            },
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
      // TODO additionalLanguages: ['rust'],
    },
    algolia: {
      apiKey: "bbaacf676920f3836ccab85fb87dd37c",
      indexName: "yew",
    },
    googleAnalytics: {
      trackingID: 'UA-175524777-1',
      anonymizeIP: true, // Should IPs be anonymized?
    },
  },
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'ja', 'zh-CN', 'zh-TW'],
  },
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl:
            'https://github.com/yewstack/yew/blob/master/website/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
};
