/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

module.exports = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  // conceptsSidebar: [{type: 'autogenerated', dirName: '.'}],

    // But you can create a sidebar manually
    sidebar: [
        {
            type: 'category',
            label: 'Getting Started',
            items: [
                {
                    type: 'category',
                    label: 'Project Setup',
                    items: [
                        'getting-started/project-setup/introduction',
                        'getting-started/project-setup/using-trunk',
                        'getting-started/project-setup/using-wasm-pack',
                    ]
                },
                "getting-started/build-a-sample-app",
                "getting-started/examples",
                "getting-started/starter-templates",
            ],
        },
        {
            type: "category",
            label: "Concepts",
            items: [
                {
                    type: "category",
                    label: "wasm-bindgen",
                    items: [
                        "concepts/wasm-bindgen/introduction",
                        "concepts/wasm-bindgen/web-sys",
                    ]
                },
                {
                    type: "category",
                    label: "Components",
                    items: [
                        "concepts/components/introduction",
                        "concepts/components/callbacks",
                        "concepts/components/properties",
                        "concepts/components/children",
                        "concepts/components/refs"
                    ],
                },
                {
                    type: "category",
                    label: "HTML",
                    items: [
                        "concepts/html/introduction",
                        "concepts/html/components",
                        "concepts/html/elements",
                        "concepts/html/events",
                        "concepts/html/classes",
                        "concepts/html/fragments",
                        "concepts/html/lists",
                        "concepts/html/literals-and-expressions"
                    ]
                },
                {
                    type: "category",
                    label: "Function Components",
                    items: [
                        "concepts/function-components/introduction",
                        "concepts/function-components/attribute",
                        "concepts/function-components/pre-defined-hooks",
                        "concepts/function-components/custom-hooks",
                    ]
                },
                "concepts/agents",
                "concepts/contexts",
                "concepts/router",
            ]
        },
        {
            type: 'category',
            label: 'Advanced topics',
            items: [
                "advanced-topics/how-it-works",
                "advanced-topics/optimizations",
                "advanced-topics/portals",
            ]
        },
        {
            type: 'category',
            label: 'More',
            items: [
                "more/debugging",
                "more/development-tips",
                "more/external-libs",
                "more/css",
                "more/testing",
                "more/roadmap",
                "more/wasm-build-tools"
            ]
        },
        {
            type: "category",
            label: "Migration guides",
            items: [
                {
                    type: "category",
                    label: "yew",
                    items: ["migration-guides/yew/from-0_18_0-to-0_19_0"],
                },
                {
                    type: "category",
                    label: "yew-agent",
                    items: ["migration-guides/yew-agent/from-0_0_0-to-0_1_0"],
                },
                {
                    type: "category",
                    label: "yew-router",
                    items: ["migration-guides/yew-router/from-0_15_0-to-0_16_0"],
                },
            ],
        },
        "tutorial"
    ],
};
