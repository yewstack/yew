module.exports = {
    migrationGuides: [
        {
            type: 'doc',
            id: 'index',
            label: 'Overview',
        },
        {
            type: 'category',
            label: 'yew',
            link: {
                type: 'generated-index',
                title: 'yew',
            },
            items: [
                'yew/from-0_22_0-to-0_23_0',
                'yew/from-0_21_0-to-0_22_0',
                'yew/from-0_20_0-to-0_21_0',
                'yew/from-0_19_0-to-0_20_0',
            ],
        },
        {
            type: 'category',
            label: 'yew-agent',
            link: {
                type: 'generated-index',
                title: 'yew-agent',
            },
            items: [
                'yew-agent/from-0_4_0-to-0_5_0',
                'yew-agent/from-0_3_0-to-0_4_0',
                'yew-agent/from-0_1_0-to-0_2_0',
                'yew-agent/from-0_0_0-to-0_1_0',
            ],
        },
        {
            type: 'category',
            label: 'yew-router',
            link: {
                type: 'generated-index',
                title: 'yew-router',
            },
            items: [
                'yew-router/from-0_19_0-to-0_20_0',
                'yew-router/from-0_16_0-to-0_17_0',
                'yew-router/from-0_15_0-to-0_16_0',
            ],
        },
    ],
}
