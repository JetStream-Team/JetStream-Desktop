<script lang="ts">
  import DisconnectedSidebar from "$lib/sidebars/disconnected.svelte" ;
  import AndroidConnectedSidebar from "$lib/sidebars/android-connected.svelte";

  let connected: Boolean = $state(false);

</script>

<div class="main">
  <div class="sidebar">
    {#if connected}
    <AndroidConnectedSidebar />
    {:else}
    <DisconnectedSidebar />
    {/if}
  </div>
  <div class="contents"></div>
</div>

<style>
  .main {
    display: flex;
  }
  .sidebar {
    width: 20rem;
    height: 100vh;
    padding: 2rem;

    background-color: var(--m3c-surface);
  }

  .contents {
    height: 100vh;
  }
</style>
