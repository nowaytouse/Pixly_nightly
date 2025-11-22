<script lang="ts">
    import type { LinkableText, Action } from '@jet-app/app-store/api/models';
    import { sanitizeHtml } from '@amp/web-app-components/src/utils/sanitize-html';
    import LinkWrapper from '~/components/LinkWrapper.svelte';

    export let item: LinkableText;

    type Fragment = {
        text: string;
        action?: Action;
        isTrailingPunctuation?: boolean;
    };

    const {
        linkedSubstrings = {},
        styledText: { rawText },
    } = item;

    // `LinkableText` items contain a `rawText` string, and an object of `linkedSubstrings`,
    // where the key of the object is the substring to replace in the `rawText` and whose value
    // is the `Action` that the link should trigger.
    //
    // That means we have to render replace the keys from `linkedSubstrings` in the `rawText`.
    // To do this, we build a regex to match all the strings that are supposed to be linked,
    // then build an array of objects representing the fully text, with the `Action` appended
    // to the fragments that need to be linked.
    const fragmentsToLink = Object.keys(linkedSubstrings);
    let fragments: Fragment[];

    if (fragmentsToLink.length === 0) {
        fragments = [{ text: rawText }];
    } else {
        // Escapes regex-sensitive characters in the text, so characters like `.` or `+` don't act as regex operators
        const cleanedFragmentsToLink = fragmentsToLink.map((fragment) =>
            fragment.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'),
        );

        const pattern = new RegExp(
            `(${cleanedFragmentsToLink.join('|')})`,
            'g',
        );

        // After we split our text into an array representing the seqence of the raw text, with the
        // linkable items as their own entries, we transform the array to contain include the linkable
        // items actions, which we then use to determine if we want to render a `LinkWrapper` or plain-text.
        fragments = rawText.split(pattern).map((fragment): Fragment => {
            const action = linkedSubstrings[fragment];

            if (action) {
                return { action, text: fragment };
            } else {
                const isTrailingPunctuation = /^[.,;:!?)\]}"”»']+$/.test(
                    fragment.trim(),
                );

                return {
                    isTrailingPunctuation,
                    text: fragment,
                };
            }
        });
    }
</script>

{#each fragments as fragment}
    {#if fragment.action}
        <LinkWrapper
            action={fragment.action}
            includeExternalLinkArrowIcon={false}
        >
            {fragment.text}
        </LinkWrapper>
    {:else if fragment.isTrailingPunctuation}
        <span class="trailing-punctuation">{fragment.text}</span>
    {:else}
        {@html sanitizeHtml(fragment.text)}
    {/if}
{/each}

<style>
    span :global(a:hover) {
        text-decoration: underline;
    }

    .trailing-punctuation {
        margin-inline-start: -0.45ch;
    }
</style>
