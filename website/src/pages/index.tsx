import React from 'react'
import Layout from '@theme/Layout'
import styles from './index.module.scss'
import useDocusaurusContext from '@docusaurus/useDocusaurusContext'
import clsx from 'clsx'
import IconExternalLink from '@theme/Icon/ExternalLink'
import Link from '@docusaurus/Link'

const Hero = () => {
    const { siteConfig } = useDocusaurusContext()

    return (
        <div className={clsx('hero shadow--lw', styles.heroHeader)}>
            <section className={styles.header}>
                <img src={siteConfig.favicon} alt="Logo" />
                <h1 className="hero__title">{siteConfig.title}</h1>
            </section>
            <section className={clsx('hero__subtitle', styles.heroSubtitle)}>
                {siteConfig.tagline}
            </section>
            <section className={styles.callToActions}>
                <Link
                    className="button button--lg button--outline button--primary margin--lg"
                    to="/docs/getting-started/introduction"
                >
                    Get Started
                </Link>
                <Link
                    className="button button--lg button--outline button--link margin--lg"
                    to="https://play.yew.rs"
                    target="_blank"
                >
                    Playground
                    <IconExternalLink />
                </Link>
            </section>
        </div>
    )
}

const FEATURES = [
    {
        header: 'Component Based',
        body: 'Features a component-based framework which makes it easy to create interactive UIs. Developers who have experience with frameworks like React and Elm should feel quite at home when using Yew.',
        to: '/docs/next/concepts/function-components',
    },
    {
        header: 'HTML macro',
        body: 'Features a macro for declaring interactive HTML with Rust expressions. Developers who have experience using JSX in React should feel quite at home when using Yew.',
        to: '/docs/next/concepts/html',
    },
    {
        header: 'Server Side Rendering',
        body: 'Features server side rendering for all the SEO and enhancements of server-rendered app while keeping the feel of an SPA',
        to: '/docs/next/advanced-topics/server-side-rendering',
    },
]

function Feature(props: { feature: (typeof FEATURES)[number] }) {
    return (
        <div className="card-demo">
            <div className="card">
                <div className="card__header">
                    <h3>{props.feature.header}</h3>
                </div>
                <div className="card__body">
                    <p>{props.feature.body}</p>
                </div>
                <div className="card__footer">
                    <Link
                        className="button button--secondary"
                        to={props.feature.to}
                    >
                        Learn more
                    </Link>
                </div>
            </div>
        </div>
    )
}

function Features() {
    return (
        <article className={clsx('padding--lg', styles.features)}>
            <h2>Features</h2>
            <section className={styles.featuresGrid}>
                {FEATURES.map((it) => (
                    <Feature feature={it} />
                ))}
            </section>
        </article>
    )
}

export default function Index() {
    const { siteConfig } = useDocusaurusContext()
    return (
        <Layout description={siteConfig.tagline}>
            <Hero />
            <Features />
        </Layout>
    )
}
