export function getCookie(name: string): string | null {
    if (typeof document === 'undefined') {
        return null;
    }

    const prefix = `${name}=`;
    const cookie = document.cookie
        .split(';')
        .map((value) => value.trimStart())
        .filter((value) => value.startsWith(prefix))[0];

    if (!cookie) {
        return null;
    }

    return cookie.substr(prefix.length);
}

export function setCookie(
    name: string,
    value: string,
    domain: string,
    expires = 0,
    path = '/',
): void {
    if (typeof document === 'undefined') {
        return undefined;
    }

    // Get any potential existing instances of this particular cookie
    const existingCookie = getCookie(name);
    let cookieValue = value;

    if (existingCookie) {
        // If exisitng cookie name does not include the value we are trying to set,
        // then add it, otherwise use the existing cookie value
        cookieValue = !existingCookie.includes(value)
            ? `${existingCookie}+${value}`
            : existingCookie;
    }

    let cookieString = `${name}=${cookieValue}; path=${path}; domain=${domain};`;

    if (expires) {
        const date = new Date();
        date.setTime(date.getTime() + expires * 24 * 60 * 60 * 1000);

        cookieString += ` expires=${date.toUTCString()};`;
    }

    document.cookie = cookieString;

    // Returning undefined because of ESLint's "consistent-return" rule
    return undefined;
}

export function clearCookie(name: string, domain: string, path = '/'): void {
    if (typeof document === 'undefined') {
        return undefined;
    }

    // Get any potential existing instances of this particular cookie
    const existingCookie = getCookie(name);

    if (existingCookie) {
        // Set the cookie's expiration date to a past date
        setCookie(name, '', domain, -1, path);
    }

    return undefined;
}
