import React from 'react';
export * from '@theme-original/NavbarItem/DefaultNavbarItem';
import OriginalNavbarItem from '@theme-original/NavbarItem/DefaultNavbarItem';
import {API_BUTTON} from "../../constants.js";
import {useLocation} from "@docusaurus/router";

export default function DefaultNavbarItem(props) {
    const location = useLocation();

    if (props.label === API_BUTTON) {
        if (location.pathname.includes('next')) {
            return <OriginalNavbarItem href={'https://yew-rs-api.web.app/next/yew'} {...props}  />
        }
    }

    return (
        <OriginalNavbarItem {...props}  />
    )
}
