type AllowedTags = Set<string>;

interface AllowedAttributes {
    [tagName: string]: Set<string>;
}

export interface SanitizeHtmlOptions {
    allowedTags?: string[];
    extraAllowedTags?: string[];
    keepChildrenWhenRemovingParent?: boolean;

    /**
     * When true, replaces all &nbsp; entities with regular spaces
     * to prevent unwanted line breaks in the rendered HTML
     */
    removeNbsp?: boolean;

    /**
     * AllowedAttributes should be an object with tag name keys and array values
     * containing all of the attributes allowed for that tag:
     *
     *    { 'p': ['class'], 'div': ['role', 'aria-hidden'] }
     *
     * The above allows ONLY the class attribute for <p> and ONLY the role and
     * aria-hidden attributes for <div>.
     */
    allowedAttributes?: {
        [tagName: string]: string[];
    };
}

export const DEFAULT_SAFE_TAGS: string[] = [
    'strong',
    'em',
    'b',
    'i',
    'u',
    'br',
];
const DEFAULT_SAFE_ATTRS = {};

/**
 * Sanitizes HTML by removing all tags and attributes that aren't explicitly allowed.
 */
export function sanitizeDocument(
    unsafeDocument: Document,
    unsafeNode: Node | DocumentFragment,
    {
        allowedTags,
        extraAllowedTags,
        allowedAttributes = DEFAULT_SAFE_ATTRS,
        keepChildrenWhenRemovingParent,
        removeNbsp,
    }: SanitizeHtmlOptions = {},
): string {
    if (allowedTags && extraAllowedTags) {
        throw new Error(
            'sanitizeHtml got both allowedTags and extraAllowedTags',
        );
    }

    const allowedTagsSet = new Set([
        ...(extraAllowedTags || []),
        ...(allowedTags || DEFAULT_SAFE_TAGS),
    ]);

    const allowedAttributeSets = {};
    for (const [tag, attributes] of Object.entries(allowedAttributes)) {
        allowedAttributeSets[tag] = new Set(attributes);
    }

    const sanitizedContainer = unsafeDocument.createElement('div');

    for (const child of [...unsafeNode.childNodes]) {
        const sanitizedChildArray = sanitizeNode(
            child as Element,
            allowedTagsSet,
            allowedAttributeSets,
            keepChildrenWhenRemovingParent,
        );
        sanitizedChildArray.forEach((node) => {
            sanitizedContainer.appendChild(node);
        });
    }

    let html = sanitizedContainer.innerHTML;

    // Replace &nbsp; with regular spaces if removeNbsp option is enabled
    if (removeNbsp) {
        html = html.replace(/&nbsp;/g, ' ');
    }

    return html;
}

function sanitizeNode(
    node: Element,
    allowedTags: AllowedTags,
    allowedAttributes: AllowedAttributes,
    keepChildrenWhenRemovingParent: boolean,
): Node[] | Element[] {
    // Plain text is safe as is
    // NOTE: The lowercase node (instead of Node) is intentional. Node is only
    //       accessible in browser. In Node.js, it depends on jsdom (which we
    //       avoid importing to exclude from the clientside vendor bundle).
    //       Instead of passing down window.Node or jsdom.Node depending on
    //       context, we rely on the fact that instances of Node (of which node
    //       will be one) will also have these constants set on them.
    if (
        ([node.TEXT_NODE, node.CDATA_SECTION_NODE] as number[]).includes(
            node.nodeType,
        )
    ) {
        return [node];
    }

    // Refuse anything that isn't a tag or one of the allowed tags
    const tagName = (node.tagName || '').toLowerCase();

    if (!allowedTags.has(tagName)) {
        // when keepChildrenWhenRemovingParent is true
        // we check children for valid nodes as well
        if (keepChildrenWhenRemovingParent) {
            return sanitizeChildren(
                node,
                allowedTags,
                allowedAttributes,
                keepChildrenWhenRemovingParent,
            );
        }
        return [];
    }

    // Reconstruct node with only the allowedAttributes and sanitize its children
    const sanitized = node.ownerDocument.createElement(tagName);
    const currentlyAllowedAttributes = allowedAttributes[tagName] || new Set();

    for (const { name, nodeValue: value } of [...node.attributes]) {
        if (currentlyAllowedAttributes.has(name)) {
            sanitized.setAttribute(name, value);
        }
    }

    const children = sanitizeChildren(
        node,
        allowedTags,
        allowedAttributes,
        keepChildrenWhenRemovingParent,
    );

    children.forEach((child) => {
        sanitized.appendChild(child);
    });

    return [sanitized];
}

const sanitizeChildren = (
    node: Element,
    allowedTags: AllowedTags,
    allowedAttributes: AllowedAttributes,
    tagsToConvertToText: boolean,
): Node[] => {
    const children = [...node.childNodes]
        .map((childNode) =>
            sanitizeNode(
                childNode as Element,
                allowedTags,
                allowedAttributes,
                tagsToConvertToText,
            ),
        )
        .flat();

    return children;
};
