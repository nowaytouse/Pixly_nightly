<script lang="ts" context="module">
    import type {
        TodayCardMedia,
        TodayCardMediaList,
    } from '@jet-app/app-store/api/models';

    export function isTodayCardMediaList(
        media: TodayCardMedia,
    ): media is TodayCardMediaList {
        return media.kind === 'list';
    }
</script>

<script lang="ts">
    import { onMount } from 'svelte';
    import SmallLockupItem from '~/components/jet/item/SmallLockupItem.svelte';

    export let media: TodayCardMediaList;

    let container: HTMLDivElement;
    let fadeTop = '0%';
    let fadeBottom = '0%';

    function calculateFadeAmounts() {
        const { scrollTop, scrollHeight, clientHeight } = container;

        fadeTop = scrollTop > 0 ? '10%' : `${scrollTop}%`;
        fadeBottom = scrollTop + clientHeight < scrollHeight - 1 ? '15%' : '0%';
    }

    onMount(() => {
        calculateFadeAmounts();
        container.addEventListener('scroll', calculateFadeAmounts);

        return () =>
            container.removeEventListener('scroll', calculateFadeAmounts);
    });
</script>

<div
    class="container"
    style:--fade-top-size={fadeTop}
    style:--fade-bottom-size={fadeBottom}
    bind:this={container}
>
    <ul>
        {#each media.lockups as item}
            <li>
                <SmallLockupItem {item} />
            </li>
        {/each}
    </ul>
</div>

<style>
    @property --fade-top-size {
        syntax: '<percentage>';
        inherits: false;
        initial-value: 0%;
    }

    @property --fade-bottom-size {
        syntax: '<percentage>';
        inherits: false;
        initial-value: 0%;
    }

    .container {
        width: 100%;
        overflow: scroll;
        padding: 0 var(--today-card-gutter);
        mask-image: linear-gradient(
            to bottom,
            transparent 0%,
            black var(--fade-top-size),
            black calc(100% - var(--fade-bottom-size)),
            transparent 100%
        );
        transition: --fade-top-size 105ms cubic-bezier(0.5, 1, 0.89, 1),
            --fade-bottom-size 420ms cubic-bezier(0.45, 0, 0.55, 1);
    }

    li {
        margin-bottom: 16px;
    }
</style>
