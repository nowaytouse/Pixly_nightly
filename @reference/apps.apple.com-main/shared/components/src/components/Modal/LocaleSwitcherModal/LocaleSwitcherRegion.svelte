<script lang="ts">
    export let regionName: string;
</script>

<div class="region-header">
    <h2>
        {regionName}
    </h2>
    <slot name="button" />
</div>
<slot name="list" />

<style lang="scss">
    .region-header {
        padding-top: 13px;
        padding-bottom: 20px;
        border-top: 1px solid var(--labelDivider);
        display: flex;
        justify-content: space-between;
        align-items: baseline;
    }

    h2 {
        margin-inline-end: 5px;
        font: var(--title-2-emphasized);
    }
</style>
