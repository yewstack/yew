/**
 * Copyright (c) 2017-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// See https://docusaurus.io/docs/site-config for all the possible
// site configuration options.

const siteConfig = {
  title: 'Yew Docs', // Title for your website.
  tagline: 'Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.',
  url: 'https://yew.rs', // Your website URL
  baseUrl: '/', // Base URL for your project */
  // For github.io type URLs, you would set the url and baseUrl like:
  //   url: 'https://facebook.github.io',
  //   baseUrl: '/test-site/',

  // Used for publishing and more
  projectName: 'yew',
  organizationName: 'yewstack',
  // For top-level user or org sites, the organization is still the same.
  // e.g., for the https://JoelMarcey.github.io site, it would be set like...
  //   organizationName: 'JoelMarcey'

  // For no header links in the top nav bar -> headerLinks: [],
  headerLinks: [
    {href: 'https://github.com/yewstack/yew/releases', label: 'Changelog'},
    {href: 'https://docs.rs/yew', label: 'API'},
    {href: 'https://discord.gg/VQck8X4', label: 'Discord'},
    {search: true},
    {languages: true},
  ],

  translationRecruitingLink: 'https://gitlocalize.com/repo/4999',

  algolia: {
    apiKey: "bbaacf676920f3836ccab85fb87dd37c",
    indexName: "yew",
  },

  gaTrackingId: 'UA-175524777-1',

  /* path to images for header/footer */
  headerIcon: 'img/logo.png',
  // footerIcon: 'img/logo.png',
  favicon: 'img/logo.png',

  /* Colors for website */
  colors: {
    primaryColor: '#008F53',
    secondaryColor: '#A4DDC8',
  },

  /* Custom fonts for website */
  /*
  fonts: {
    myFont: [
      "Times New Roman",
      "Serif"
    ],
    myOtherFont: [
      "-apple-system",
      "system-ui"
    ]
  },
  */

  highlight: {
    // Highlight.js theme to use for syntax highlighting in code blocks.
    theme: 'default',
  },
  // prism manages to actually highlight our code properly
  usePrism: ["rust"],

  // Add custom scripts here that would be placed in <script> tags.
  // scripts: ['https://buttons.github.io/buttons.js'],

  stylesheets: [
    'https://fonts.googleapis.com/css?family=Roboto:400,400i,500,700',
  ],

  // On page navigation for the current documentation page.
  onPageNav: 'separate',
  // No .html extensions for paths.
  cleanUrl: true,

  // Open Graph and Twitter card images.
  ogImage: 'img/Rollup.jpg',
  twitterImage: 'img/Rollup.jpg',
  twitterUsername: 'yewstack',

  // For sites with a sizable amount of content, set collapsible to true.
  // Expand/collapse the links and subcategories under categories.
  // docsSideNavCollapsible: true,

  // Show documentation's last contributor's name.
  // enableUpdateBy: true,

  // Show documentation's last update time.
  enableUpdateTime: true,

  // You may provide arbitrary config keys to be used as needed by your
  // template. For example, if you need your repo's URL...
  repoUrl: 'https://github.com/yewstack/yew',
  markdownPlugins: [
    // Highlight admonitions.
    require('remarkable-admonitions')({icon: 'svg-inline'})
  ]
};

module.exports = siteConfig;
