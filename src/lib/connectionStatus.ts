import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export async function updateHoverMessage(isConnected: boolean) {
    await invoke('update_tray_tooltip', { connected: isConnected });
}

export async function listenConnectionStatus() {
    await listen<boolean>('connection_status', (event) => {
        updateHoverMessage(event.payload);
    });
}