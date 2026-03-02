import React from 'react'
import { useLocation } from '@docusaurus/router'
export * from '@theme-original/NavbarItem/DefaultNavbarItem'
import OriginalNavbarItem from '@theme-original/NavbarItem/DefaultNavbarItem'
import { API_BUTTON } from '../../constants.js'

const VERSION_REGEX = /\/docs\/(0.([0-9]+)(\.[0-9]+)?|next)?/
const API_BASE_URLS = {
    next: 'https://api.yew.rs/next/yew',
    default: 'https://docs.rs/yew',
}

/**
 * @returns {string}
 */
const useVersion = () => {
    const location = useLocation()
    const match = location.pathname.match(VERSION_REGEX)
    return match ? (match[1] ?? '') : ''
}

/**
 * @param {string} version
 * @returns {string}
 */
const getApiUrl = (version) => {
    if (version === 'next') {
        return API_BASE_URLS.next
    }
    return version
        ? `${API_BASE_URLS.default}/${version}`
        : API_BASE_URLS.default
}

/**
 * @param {Object} props
 * @param {string} props.label
 * @returns {React.ReactElement}
 */
export default function DefaultNavbarItem(props) {
    const { label, ...restProps } = props

    if (label === API_BUTTON) {
        const version = useVersion()
        const href = getApiUrl(version)
        return <OriginalNavbarItem {...restProps} label={label} href={href} />
    }

    return <OriginalNavbarItem {...props} />
}
