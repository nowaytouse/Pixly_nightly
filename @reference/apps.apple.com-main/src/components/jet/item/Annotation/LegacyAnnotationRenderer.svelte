<script lang="ts">
    import { isSome } from '@jet/environment';
    import {
        type AnnotationItem,
        type Action,
        isFlowAction,
    } from '@jet-app/app-store/api/models';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let items: AnnotationItem[];
    export let linkAction: Action | undefined;

    const shouldRenderAsDefinitionList = (items: AnnotationItem[]) =>
        !!items[0]?.heading;

    const shouldRenderAsOrderedList = (items: AnnotationItem[]) =>
        !!items[0]?.textPairs;

    const shouldRenderAsUnorderedList = (items: AnnotationItem[]) =>
        !items[0]?.text;

    const shouldRenderAsDefinitionListWithHeading = (items: AnnotationItem[]) =>
        items[0]?.text && items[1]?.heading;
</script>

{#if shouldRenderAsDefinitionList(items)}
    <dl class="secondary-definition-list">
        {#each items as annotationItem}
            <dt>{annotationItem.heading}</dt>
            <dd>{annotationItem.text}</dd>
        {/each}
    </dl>
{:else if shouldRenderAsOrderedList(items)}
    <ol>
        {#each items as annotationItem}
            {#if annotationItem.textPairs}
                {#each annotationItem.textPairs as [text, subtext]}
                    <li>
                        <span class="text">{text}</span>
                        <span class="subtext">{subtext}</span>
                    </li>
                {/each}
            {:else}
                <li>{annotationItem.text}</li>
            {/if}
        {/each}
    </ol>
{:else if shouldRenderAsUnorderedList(items)}
    <ul>
        {#each items as annotationItem}
            <li>
                <span class="text">
                    {annotationItem.text}
                </span>
            </li>
        {/each}
    </ul>
{:else if shouldRenderAsDefinitionListWithHeading(items)}
    {@const [heading, ...remainingItems] = items}
    <dd>
        <p class="secondary-definition-list-heading">{heading.text}</p>

        <dl class="secondary-definition-list">
            {#each remainingItems as annotationItem}
                <dt>{annotationItem.heading}</dt>
                <dd>{annotationItem.text}</dd>
            {/each}
        </dl>
    </dd>
{:else}
    <dd>
        <ul>
            {#each items as annotationItem}
                <li>{annotationItem.text}</li>
            {/each}
        </ul>
        {#if isSome(linkAction) && isFlowAction(linkAction)}
            <LinkWrapper action={linkAction}>
                {linkAction.title}
            </LinkWrapper>
        {/if}
    </dd>
{/if}

<style>
    dt {
        color: var(--systemSecondary);
        font: var(--body-tall);
    }

    dd {
        white-space: pre-line;
        font: var(--body-tall);
    }

    ol {
        counter-reset: section;
    }

    ol li {
        display: table-row;
        font: var(--body-tall);
    }

    ol li::before {
        counter-increment: section;
        content: counter(section) '.';
        display: table-cell;
        padding-inline-end: 6px;
    }

    ol li .text {
        display: table-cell;
        width: 100%;
    }

    ol li .subtext {
        display: table-cell;
    }

    .secondary-definition-list-heading {
        margin-bottom: 16px;
    }

    .secondary-definition-list dt {
        color: var(--systemPrimary);
        font: var(--body-emphasized);
    }

    .secondary-definition-list dd:not(:last-of-type) {
        margin-bottom: 16px;
    }

    dd li:not(:last-of-type) {
        margin-bottom: 16px;
    }

    dd :global(a) {
        color: var(--keyColor);
        text-decoration: none;
    }

    dd :global(a:hover) {
        text-decoration: underline;
    }
</style>
