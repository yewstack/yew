/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation
 The sidebars can be generated from the filesystem, or explicitly defined here.
 Create as many sidebars as you want.
 */

module.exports = {
    docs: [
        {
            type: 'category',
            label: 'Getting Started',
            link: { type: 'doc', id: 'getting-started/introduction' },
            items: [
                'getting-started/build-a-sample-app',
                'getting-started/examples',
                'getting-started/editor-setup',
            ],
        },
        {
            type: 'category',
            label: 'Concepts',
            link: {
                type: 'generated-index',
                title: 'Yew concepts',
                description: 'Learn about the important Yew concepts!',
            },
            items: [
                {
                    type: 'category',
                    label: 'Using Basic Web Technologies In Yew',
                    link: {
                        type: 'generated-index',
                        title: "Yew's Take on Basic Web Technologies",
                        description:
                            'Yew centrally operates on the idea of keeping everything that a reusable piece of UI may need' +
                            'in one place - rust files, while also keeping the underlying technology accessible where necessary. ' +
                            'Explore further to fully grasp what we mean by these statements:',
                    },
                    items: [
                        'concepts/basic-web-technologies/html',
                        'concepts/basic-web-technologies/css',
                        'concepts/basic-web-technologies/js',
                        'concepts/basic-web-technologies/wasm-bindgen',
                        'concepts/basic-web-technologies/web-sys',
                    ],
                },
                {
                    type: 'category',
                    label: 'Components',
                    link: {
                        type: 'doc',
                        id: 'concepts/function-components/introduction',
                    },
                    items: [
                        'concepts/function-components/properties',
                        'concepts/function-components/callbacks',
                        'concepts/function-components/children',
                        'concepts/function-components/pure-components',
                        {
                            type: 'category',
                            label: 'Hooks',
                            link: {
                                type: 'doc',
                                id: 'concepts/function-components/hooks/introduction',
                            },
                            items: [
                                'concepts/function-components/hooks/custom-hooks',
                            ],
                        },
                        'concepts/function-components/node-refs',
                        'concepts/function-components/state',
                        'concepts/function-components/communication',
                        'concepts/function-components/generics',
                    ],
                },
                {
                    type: 'category',
                    label: 'HTML',
                    link: { type: 'doc', id: 'concepts/html/introduction' },
                    items: [
                        'concepts/html/components',
                        'concepts/html/elements',
                        'concepts/html/events',
                        'concepts/html/classes',
                        'concepts/html/fragments',
                        'concepts/html/lists',
                        'concepts/html/literals-and-expressions',
                        'concepts/html/conditional-rendering',
                    ],
                },
                'concepts/agents',
                'concepts/contexts',
                'concepts/router',
                'concepts/suspense',
            ],
        },
        {
            type: 'category',
            label: 'Advanced topics',
            link: {
                type: 'generated-index',
                title: 'Advanced topics',
                description:
                    'Learn about the advanced topics and inner workings of Yew!',
            },
            items: [
                'advanced-topics/how-it-works',
                {
                    type: 'category',
                    label: 'Struct Components',
                    link: {
                        type: 'doc',
                        id: 'advanced-topics/struct-components/introduction',
                    },
                    items: [
                        'advanced-topics/struct-components/hoc',
                        'advanced-topics/struct-components/lifecycle',
                        'advanced-topics/struct-components/scope',
                        'advanced-topics/struct-components/callbacks',
                        'advanced-topics/struct-components/properties',
                        'advanced-topics/struct-components/refs',
                    ],
                },
                'advanced-topics/children',
                'advanced-topics/optimizations',
                'advanced-topics/portals',
                'advanced-topics/server-side-rendering',
                'advanced-topics/immutable',
            ],
        },
        {
            type: 'category',
            label: 'More',
            link: {
                type: 'generated-index',
                title: 'Miscellaneous',
            },
            items: [
                'more/debugging',
                'more/deployment',
                'more/css',
                'more/testing',
                'more/roadmap',
            ],
        },
        {
            type: 'category',
            label: 'Migration guides',
            items: [
                {
                    type: 'category',
                    label: 'yew',
                    items: ['migration-guides/yew/from-0_18_0-to-0_19_0'],
                },
                {
                    type: 'category',
                    label: 'yew-agent',
                    items: ['migration-guides/yew-agent/from-0_0_0-to-0_1_0'],
                },
                {
                    type: 'category',
                    label: 'yew-router',
                    items: [
                        'migration-guides/yew-router/from-0_15_0-to-0_16_0',
                    ],
                },
            ],
        },
    ],
    api: [{ type: 'autogenerated', dirName: 'tutorial' }],
}
