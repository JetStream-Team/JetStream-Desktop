import { invoke } from '@tauri-apps/api/core';

export async function updateHoverMessage(isConnected: boolean) {
    await invoke('update_tray_tooltip', { connected: isConnected });
}