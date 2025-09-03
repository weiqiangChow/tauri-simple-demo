import { invoke } from "@tauri-apps/api/core";

type IPlatform = 'darwin' | 'win32' | 'linux' | 'unknown'

export function useInit() {
    const initPlatform = async () => {
        const platform: IPlatform = await invoke('platform_cmd')
        document.documentElement.classList.add(`platform-${platform}`)
    }
    return {
        initPlatform,
    }
}
