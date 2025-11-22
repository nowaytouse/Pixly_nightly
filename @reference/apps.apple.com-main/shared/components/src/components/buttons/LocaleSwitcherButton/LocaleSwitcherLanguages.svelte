<script lang="ts">
    import type { Language } from '@amp/web-app-components/src/components/buttons/LocaleSwitcherButton/types';
    export let translateFn: (
        str: string,
        values?: Record<string, string | number>,
    ) => string;
    export let otherLanguages: Language[];

    const handleClick = (otherLanguage: string) => {
        const url = new URL(window.location.href);
        url.searchParams.set('l', otherLanguage);
        window.location.assign(`${url.pathname}${url.search}`);
    };
</script>

{#if otherLanguages.length > 0}
    <ul class:languages-new-line={otherLanguages.length >= 6}>
        {#each otherLanguages as otherLanguage}
            {#if otherLanguage.tag && otherLanguage.name}
                <li>
                    <a
                        on:click|preventDefault={() =>
                            handleClick(otherLanguage.tag)}
                        href={`?l=${otherLanguage.tag}`}
                        aria-label={translateFn(
                            'AMP.Shared.LocaleSwitcher.SwitchLanguage',
                            { language: otherLanguage.name },
                        )}
                        data-testid={`other-language-${otherLanguage.tag}`}
                    >
                        {otherLanguage.name}
                    </a>
                </li>
            {/if}
        {/each}
    </ul>
{/if}

<style lang="scss">
    a {
        --linkColor: var(--systemSecondary);
        white-space: nowrap;
        padding-inline-end: 10px;
    }

    ul {
        display: flex;
        flex-wrap: wrap;
        padding-inline-start: 10px;

        &.languages-new-line {
            @media (--range-small-down) {
                padding-inline-start: 0;

                li {
                    &:first-of-type {
                        a {
                            padding-inline-start: 0;
                        }

                        &::before {
                            content: '';
                            height: 100%;
                            border-inline-start: none;
                        }
                    }
                }
            }
        }

        li {
            margin-top: 6px;
            display: inline-flex;
            line-height: 1;
            vertical-align: middle;

            &:first-of-type {
                a {
                    padding-inline-start: 10px;
                }

                &::before {
                    content: '';
                    height: 100%;
                    border-inline-start: 1px solid var(--systemQuaternary);
                }
            }

            &::after {
                border-inline-start: 1px solid var(--systemQuaternary);
                content: '';
                padding-inline-end: 10px;
            }

            &:last-child::after {
                content: none;
            }
        }
    }
</style>
