import { useRuntimeConfig } from "#imports";

export function apiURL(url: string) {
    if (url.startsWith("/"))
        url = url.slice(1);
    return useRuntimeConfig().public.apiURL + url;
}