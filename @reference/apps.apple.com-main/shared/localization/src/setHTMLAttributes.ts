import { getLocAttributes } from './getLocAttributes';

/**
 * sets Language attributes to HTML tag.
 * @param {string} language
 * @returns {void}
 */
export function setHTMLAttributes(language: string): void {
    if (typeof window === 'undefined') return;
    const attributes = getLocAttributes(language);

    for (let [attribute, value] of Object.entries(attributes)) {
        window.document.documentElement.setAttribute(attribute, value);
    }
}
