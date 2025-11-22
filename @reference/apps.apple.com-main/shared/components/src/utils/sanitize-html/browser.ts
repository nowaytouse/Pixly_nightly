// Browser ONLY logic. Must have the same exports as server.ts
// See: docs/isomorphic-imports.md

import { type SanitizeHtmlOptions, sanitizeDocument } from './common';

export { type SanitizeHtmlOptions, DEFAULT_SAFE_TAGS } from './common';

// Shared DOMParser instance (avoids creating a new one for each sanitization)
let parser = null;

export function sanitizeHtml(
    input: string,
    options: SanitizeHtmlOptions = {},
): string {
    if (!input) {
        return input;
    }

    if (!parser) {
        parser = new DOMParser();
    }

    const unsafeDocument = parser.parseFromString(`${input}`, 'text/html');
    const unsafeNode = unsafeDocument.body;
    return sanitizeDocument(unsafeDocument, unsafeNode, options);
}
