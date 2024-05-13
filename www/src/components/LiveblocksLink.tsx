"use client";

import DefaultLink from "next/link";
import { useSearchParams } from "next/navigation";
import { ComponentProps, useMemo } from "react";


const ABSOLUTE_URL_REGEX = /^[a-zA-Z][a-zA-Z\d+\-.]*?:/;
const PLACEHOLDER_BASE_URL = "https://localhost:3000";


export function setQueryParams(
    url: string,
    params: Record<string, string | number | boolean | null | undefined>
) {
    const isAbsolute = ABSOLUTE_URL_REGEX.test(url);
    const absoluteUrl = new URL(
        url,
        isAbsolute ? undefined : PLACEHOLDER_BASE_URL
    );

    for (const [param, value] of Object.entries(params)) {
        if (value === undefined) {
            absoluteUrl.searchParams.delete(param);
        } else {
            absoluteUrl.searchParams.set(param, String(value));
        }
    }

    return isAbsolute
        ? absoluteUrl.href
        : absoluteUrl.href.replace(PLACEHOLDER_BASE_URL, "");
}


interface LinkProps extends Omit<ComponentProps<typeof DefaultLink>, "href"> {
    href?: string;
}

const PRESERVED_QUERY_PARAMS = ["exampleId", "examplePreview"];

/**
 * This version of `Link` is used to preserve query parameters when deploying
 * an example on liveblocks.io. You can ignore it completely if you run the
 * example locally, and use the default `next/link` component instead.
 */
export function LiveblocksLink({ href, ...props }: LinkProps) {
    const params = useSearchParams();
    const hrefWithQueryParams = useMemo(() => {
        if (!href) {
            return;
        }

        const preservedQueryParams: Record<string, string> = {};

        params.forEach((value, param) => {
            if (PRESERVED_QUERY_PARAMS.includes(param)) {
                preservedQueryParams[param] = value;
            }
        });

        return setQueryParams(href, preservedQueryParams);
    }, [href, params]);

    return hrefWithQueryParams ? (
        <DefaultLink href={hrefWithQueryParams} {...props} />
    ) : (
        <a {...props} />
    );
}
