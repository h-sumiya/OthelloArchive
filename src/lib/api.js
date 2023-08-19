import { invoke } from "@tauri-apps/api/tauri"

export async function greet(name) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    return await invoke("greet", { name })
}

export async function dataToBit(data) {
    return await invoke("data_to_bit", { data })
}

export async function textToData(text) {
    return await invoke("text_to_data", { text })
}