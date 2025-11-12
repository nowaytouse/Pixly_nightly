<script lang="ts">
    import { BASE, getFontURL } from '@amp/web-apps-fonts';

    export let language: string;

    $: fontURL = getFontURL(language);
</script>

<svelte:head>
    <link rel="preconnect" href={BASE} crossorigin="anonymous" />

    <link
        rel="stylesheet"
        as="style"
        href={fontURL}
        type="text/css"
        referrerpolicy="strict-origin-when-cross-origin"
    />
</svelte:head>
