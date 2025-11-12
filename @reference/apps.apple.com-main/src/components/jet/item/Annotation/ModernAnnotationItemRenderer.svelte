<script lang="ts">
    import type { AnnotationItem_V3 } from '@jet-app/app-store/api/models';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import SystemImage, {
        isSystemImageArtwork,
    } from '~/components/SystemImage.svelte';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let items: AnnotationItem_V3[];
    export let summary: string | undefined;

    const formatStyledText = (text: string): string => {
        return (
            text
                // Replace \n with <br>
                .replace(/\n/g, '<br>')
                // Replace **text** with <strong>text</strong>
                .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
        );
    };
</script>

<ul>
    {#each items as annotationItem}
        <li>
            {#if annotationItem.$kind === 'textEncapsulation'}
                <div class="text-encapsulation">
                    {annotationItem.text}
                </div>
            {:else if annotationItem.$kind === 'linkableText'}
                <div class="styled-text">
                    {@html sanitizeHtml(
                        formatStyledText(
                            annotationItem.linkableText.styledText.rawText,
                        ),
                    )}
                </div>
            {:else if annotationItem.$kind === 'artwork'}
                {#if isSystemImageArtwork(annotationItem.artwork)}
                    <div class="artwork-wrapper" aria-label={summary}>
                        <SystemImage artwork={annotationItem.artwork} />
                    </div>
                {/if}
            {:else if annotationItem.$kind === 'textPair'}
                <div class="text-pair">
                    <span>{annotationItem.leadingText}</span>
                    <span>
                        {annotationItem.trailingText}
                    </span>
                </div>
            {:else if annotationItem.$kind === 'button'}
                <div class="button-wrapper">
                    <LinkWrapper action={annotationItem.action}>
                        {annotationItem.action.title}
                    </LinkWrapper>
                </div>
            {:else if annotationItem.$kind === 'spacer'}
                <div class="spacer" />
            {/if}
        </li>
    {/each}
</ul>

<style>
    li {
        font: var(--body-tall);
    }

    .styled-text :global(strong) {
        color: var(--systemPrimary);
        font: var(--body-emphasized);
    }

    .text-encapsulation {
        width: fit-content;
        color: var(--keyColor);
        border: 1px solid;
        border-radius: 3px;
        padding-inline: 3px;
        border-color: var(--keyColor);
        margin-block: 3px;
    }

    .artwork-wrapper :global(svg) {
        height: 18px;
        width: 18px;
        margin-top: 4px;
    }

    .spacer {
        height: 16px;
    }

    .button-wrapper :global(a) {
        color: var(--keyColor);
        text-decoration: none;
    }

    .button-wrapper :global(a:hover) {
        text-decoration: underline;
    }

    .button-wrapper :global(a) :global(.external-link-arrow) {
        width: 7px;
        height: 7px;
        fill: var(--keyColor);
        margin-top: 3px;
    }

    .text-pair {
        display: flex;
        justify-content: space-between;
    }
</style>
