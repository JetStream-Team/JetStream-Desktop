<script lang="ts">
    let { title, desc } = $props()
</script>

<div class="notification">
    <h4>{title}</h4>
    <p>{desc}</p>
</div>

<style>
    .notification {
        background-color: var(--m3c-surface-container-high);
        border-radius: var(--rounding);

        width: 100%;
        padding: 0.5rem;

        margin-bottom: 0.5rem;
    }

    h4 {
        font-weight: 500;
        margin: 0.5rem;
    }

    p {
        margin: 0.5rem;
    }
</style>