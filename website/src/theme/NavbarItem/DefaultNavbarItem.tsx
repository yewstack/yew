import React from 'react'
export * from '@theme-original/NavbarItem/DefaultNavbarItem'
import OriginalNavbarItem from '@theme-original/NavbarItem/DefaultNavbarItem'
import { API_BUTTON } from '../../constants.js'
import { useLocation } from '@docusaurus/router'

const regex = /\/docs\/(0.([0-9]+)(\.[0-9]+)?|next)?/

const useVersion = () => {
    const location = useLocation()
    const match = location.pathname.match(regex)
    return match ? match[1] ?? '' : ''
}

export default function DefaultNavbarItem(props) {
    const version = useVersion()

    if (props.label === API_BUTTON) {
        const href =
            version === 'next'
                ? 'https://api.yew.rs/next/yew'
                : `https://docs.rs/yew/${version}`
        return <OriginalNavbarItem {...props} href={href} />
    }

    return <OriginalNavbarItem {...props} />
}
