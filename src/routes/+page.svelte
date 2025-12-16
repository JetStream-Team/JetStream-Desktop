<script lang="ts">
    import DisconnectedSidebar from "$lib/sidebars/disconnected.svelte";
    import AndroidSidebar from "$lib/sidebars/android.svelte";
    import DisconnectedContent from "$lib/content/disconnected.svelte";
    import AndroidContent from "$lib/content/android.svelte";

    let isConnected: Boolean = $state(true);
</script>

<div class="main">
    <div class="sidebar">
        {#if isConnected}
            <AndroidSidebar />
        {:else}
            <DisconnectedSidebar />
        {/if}
    </div>
    <div class="contents">
        {#if isConnected}
            <AndroidContent />
        {:else}
            <DisconnectedContent />
        {/if}
    </div>
</div>

<style>
    .main {
        display: flex;
        box-sizing: border-box;
    }
    .sidebar {
        width: 20rem;
        height: 100vh;

        background-color: var(--m3c-surface);
    }

    .contents {
        height: 100vh;
    }
</style>
