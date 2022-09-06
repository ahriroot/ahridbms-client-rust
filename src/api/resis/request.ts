import { invoke } from "@tauri-apps/api/tauri"
import { Response } from '@/types/redis'

const request = async <T>(command: string, params: any): Promise<T> => {
    let res = await invoke<Response<T>>(command, params)
    if (res.code !== 10000) {
        if (res.code < 50000) {
            window.$message.warning(res.msg)
        } else {
            window.$message.error(res.msg)
        }
        return Promise.reject(res.msg)
    }
    return res.data
}

export default request
