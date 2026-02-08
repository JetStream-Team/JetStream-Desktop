<script lang="ts">
    import Notification from "./notification.svelte";

    let wifi_on = $state(true);
    let bluetooth_on = $state(true);
    let location_on = $state(true);
    let battery_charging = $state(true);

    let notifications = [
        ["Hello 1", "This is a sample notification 1"],
        ["Hello 2", "This is a sample notification 2"],
        ["Hello 3", "This is a sample notification 3"],
    ];
</script>

<div class="main">
    <div class="status">
        <div class="phone">
            <div class="phone-frame"></div>
        </div>
        <div class="details">
            <div class="text">
                <div class="name">Sample Device</div>
                <div class="ip">127.0.0.1</div>
            </div>
            <div class="icons">
                <div class="wifi">
                    {#if wifi_on}
                        <img src="/icons/wifi_on.svg" alt="Wifi On" />
                    {:else}
                        <img src="/icons/wifi_off.svg" alt="Wifi Off" />
                    {/if}
                </div>
                <div class="bluetooth">
                    {#if bluetooth_on}
                        <img src="/icons/bluetooth_on.svg" alt="Bluetooth On" />
                    {:else}
                        <img src="/icons/bluetooth_off.svg" alt="Bluetooth Off" />
                    {/if}
                </div>
                <div class="location">
                    {#if location_on}
                        <img src="/icons/location_on.svg" alt="Location On" />
                    {:else}
                        <img src="/icons/location_off.svg" alt="Location Off" />
                    {/if}
                </div>
                <div class="battery">
                    {#if battery_charging}
                        <img src="/icons/battery_charging.svg" alt="Battery Charging" />
                    {:else}
                        <img src="/icons/battery_5.svg" alt="Battery Full" />
                    {/if}
                </div>
            </div>
        </div>
    </div>
    <div class="notifications">
        {#if notifications.length === 0}
            <div class="no-notif">
                No Notifications
            </div>
        {:else}
            {#each notifications as notif }
            <Notification title={notif[0]} desc={notif[1]}/>      
            {/each}
        {/if}
    </div>
</div>

<style>
    .main {
        width: 100%;
        height: 100%;

        padding: 1rem;

        display: flex;
        flex-direction: column;
    }

    .status {
        width: 100%;
        height: fit-content;
        margin-bottom: 1rem;

        display: flex;

        background-color: var(--m3c-surface-bright);
        border-radius: var(--rounding);
    }

    .phone {
        padding: 1rem;

        .phone-frame {
            height: 8rem;
            width: 4.5rem;

            border: 0.3rem solid white;
            border-radius: 0.8rem;
        }
    }

    .details {
        width: 100%;
        margin: 1rem;

        display: flex;
        flex-direction: column;
        justify-content: space-between;

        .text {
            margin-bottom: 1rem;
            line-height: 1.5rem;
        }

        .icons {
            display: flex;
            width: 100%;
            justify-content: space-between;

            img {
                height: 1.5rem;
            }
        }
    }

    .notifications {
        height: 100%;

        .no-notif {
            margin: auto;
            padding-top: 1rem;
        }
    }
</style>
