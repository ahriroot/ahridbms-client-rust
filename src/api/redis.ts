import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"

const setString = async (params: InvokeArgs): Promise<string> => {
    let res = await invoke<string>('set_string', params)
    return res
}

export { setString }

