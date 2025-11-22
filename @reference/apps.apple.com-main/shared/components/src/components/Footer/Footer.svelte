<script lang="ts" context="module">
    export type Translate = (
        str: string,
        options?: Record<string, string | number>,
    ) => string;
</script>

<script lang="ts">
    import type { FooterItem } from '@amp/web-app-components/src/components/Footer/types';
    /**
     * Available CSS Vars:
     * --footerBg
     *
     * StyleKit Vars:
     * --keyColor
     * --systemPrimary
     * --systemSecondary
     * --systemQuaternary
     */

    /**
     * translate function provided by the parent app.
     */
    export let translateFn: Translate;
    /**
     * A list of links to be in the footer
     * @type {Array<FooterItem>}
     */
    export let footerItems: FooterItem[];

    const year = new Date().getFullYear().toString();
</script>

<footer data-testid="footer">
    <div class="footer-secondary-slot">
        <slot name="secondary-content" />
    </div>

    <div class="footer-contents">
        <p>
            <span dir="ltr">
                <span dir="auto"
                    >{translateFn('AMP.Shared.Footer.CopyrightYear', {
                        year,
                    })}</span
                >
                <a
                    href={translateFn('AMP.Shared.Footer.Apple.URL')}
                    rel="noopener"
                    ><span dir="auto"
                        >{translateFn('AMP.Shared.Footer.Apple.Text')}</span
                    ></a
                >
            </span>
            <span dir="auto"
                >{translateFn('AMP.Shared.Footer.AllRightsReserved')}</span
            >
        </p>
        <ul>
            {#each footerItems as { url, locKey, id } (id)}
                <li data-testid={id}>
                    <a href={translateFn(url)} rel="noopener" dir="auto">
                        {translateFn(locKey)}
                    </a>
                </li>
            {/each}
        </ul>
    </div>
</footer>

<style lang="scss">
    @use '@amp/web-shared-styles/sasskit-stylekit/ac-sasskit-config';
    @use 'ac-sasskit/core/typography/specs' as *;
    @use 'ac-sasskit/core/selectors' as *;
    @use 'ac-sasskit/core/viewports' as *;
    @use 'amp/stylekit/core/fonts' as *;
    @use 'amp/stylekit/core/specs' as *;
    @use 'amp/stylekit/modules/fontsubsets/core' as *;
    @use '@amp/web-shared-styles/app/core/viewports' as *;
    @use '@amp/web-shared-styles/app/core/globalvars' as *;

    $footer-height-sidebar-visible: 88px;
    $footer-height-xsmall: 147px;
    $footer-height-small: 88px;
    $footer-vertical-padding-xsmall: var(--footerVerticalPadding, 15px);
    $footer-vertical-padding-small: var(--footerVerticalPadding, 14px);

    footer {
        flex-shrink: 0;
        min-height: $footer-height-xsmall;
        padding: $footer-vertical-padding-xsmall var(--bodyGutter);
        background-color: var(--footerBg);
        display: block;

        @include typespec(Footnote);

        // Footer.svelte should use viewport mixins for media queries
        // this allows for cross compatibility with apps that may have
        // differing xsmall vs small viewports set up
        @include viewport('range:sidebar:hidden down') {
            padding-bottom: $global-player-bar-height +
                $footer-vertical-padding-xsmall;
        }

        @include viewport(small) {
            min-height: $footer-height-sidebar-visible;
            padding-top: $footer-vertical-padding-small;
            padding-bottom: $footer-vertical-padding-small;

            @include typespec(Subhead);
        }

        @include viewport(xlarge) {
            align-content: flex-start;
            align-items: baseline;
            display: var(--footerDisplay, flex);
            justify-content: space-between;
        }

        @include feature-detect(is-footer-hidden) {
            display: none;
        }

        // Hide Footer for Replay Highlights
        :global(.maximize-content-area) & {
            display: none;
        }
    }

    .footer-contents {
        @include viewport(small) {
            order: 1;
        }

        p {
            margin-bottom: 5px;
            color: var(--systemSecondary);
        }

        a {
            --linkColor: var(--systemPrimary);
        }

        ul {
            display: flex;
            flex-wrap: wrap;
        }

        li {
            display: inline-flex;
            line-height: 1;
            margin-top: 6px;
            vertical-align: middle;

            a {
                height: 100%;
                padding-inline-end: 10px;
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

    .footer-secondary-slot {
        --linkColor: var(--systemSecondary);
        order: 1;
        // Font subsets for Geos prevents `SF Pro` Web Font from being
        // downloaded after `BlinkMacSystemFont` fails in Chrome.
        font-family: font-family-locale(en-WW, geos);

        @each $lang, $font in font-family(geos) {
            @if $lang != en-WW {
                :global([lang]:lang(#{$lang})) & {
                    font-family: $font;
                }
            }
        }

        @include viewport(small) {
            order: 2;
        }

        @include viewport('range:xsmall down') {
            min-width: auto;
        }
    }
</style>
