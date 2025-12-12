<script>
    import { Button } from "m3-svelte";
</script>

<div class="main">
    <img src="icons/devices_disconnected.svg" alt="">
    <p>No devices are connected</p>
    <Button onclick={() => alert("!")}>Connect</Button>
</div>

<style>
    .main {
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    img {
        height: 3rem;
        display: block;
    }
</style>