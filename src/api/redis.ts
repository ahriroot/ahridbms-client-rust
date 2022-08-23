import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"


interface SetStringArgs extends InvokeArgs {
    conn: any
    key: string
    value: string
    ttl: number
}

interface RPushArgs extends InvokeArgs {
    conn: any
    key: string
    value: string[]
    ttl: number
}

interface SaddArgs extends InvokeArgs {
    conn: any
    key: string
    value: string[]
    ttl: number
}

interface ZaddArgs extends InvokeArgs {
    conn: any
    key: string
    value: {
        score: number
        value: string
    }[]
    ttl: number
}

interface HmsetArgs extends InvokeArgs {
    conn: any
    key: string
    value: {
        key: string
        value: string
    }[]
    ttl: number
}

const setString = async (params: SetStringArgs): Promise<string> => {
    let res = await invoke<string>('set_string', params)
    return res
}

const rpush = async (params: RPushArgs): Promise<number> => {
    let res = await invoke<number>('rpush', params)
    return res
}

const sadd = async (params: SaddArgs): Promise<number> => {
    let res = await invoke<number>('sadd', params)
    return res
}

const zadd = async (params: ZaddArgs): Promise<number> => {
    let res = await invoke<number>('zadd', params)
    return res
}

const hmset = async (params: HmsetArgs): Promise<string> => {
    let res = await invoke<string>('hmset', params)
    return res
}

export { setString, rpush, sadd, zadd, hmset }

