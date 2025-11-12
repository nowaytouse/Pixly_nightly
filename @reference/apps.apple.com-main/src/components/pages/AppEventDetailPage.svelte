<script lang="ts" context="module">
    import type { DefaultPageRequirements } from './DefaultPage.svelte';
</script>

<script lang="ts">
    import ShelfComponent from '~/components/jet/shelf/Shelf.svelte';

    export let page: DefaultPageRequirements;
</script>

<div class="app-event-detail-page-container">
    <div class="shelf-container">
        {#each page.shelves as shelf}
            <ShelfComponent {shelf} />
        {/each}
    </div>
</div>

<style>
    .app-event-detail-page-container {
        display: flex;
        flex-direction: column;
        flex-grow: 1;
        padding: 0 var(--bodyGutter);
    }

    .shelf-container {
        display: flex;
        flex: 1;
        flex-direction: column;
        width: 100%;
        max-width: 900px;
        margin: 0 auto;

        @media (--range-small-up) {
            justify-content: center;
        }
    }

    .shelf-container :global(.shelf) {
        margin: 0;
        padding: var(--bodyGutter) 0 0;
    }
</style>
