import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { AllKeys, Response } from '@/types/redis'
import { reset } from "./resis/string"
import { lpush, rpush, lpop, rpop, lset } from "./resis/list"
import { sadd, srem } from "./resis/set"
import { zadd, zrem } from "./resis/zset"
import { hset, hdel } from "./resis/hash"
import { json_set } from "./resis/json"


interface KeysArgs extends InvokeArgs {
    conn: any
    arg: string
    db: string
}

interface SetStringArgs extends InvokeArgs {
    conn: any
    key: string
    value: string
    ttl: number
    db: string
}

interface DelArgs extends InvokeArgs {
    conn: any
    key: string
    db: string
}

interface GetArgs extends InvokeArgs {
    conn: any
    key: string
    db: string
}

interface ExpireArgs extends InvokeArgs {
    conn: any
    key: string
    ttl: number
    db: string
}

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

const info = async (params: InvokeArgs): Promise<string> => {
    let res = await request<any>('plugin:redis|info', params)
    return res
}

const keys = async (params: KeysArgs): Promise<AllKeys[]> => {
    let res = await request<AllKeys[]>('plugin:redis|keys', params)
    return res
}

const set = async (params: SetStringArgs): Promise<string> => {
    let res = await request<string>('plugin:redis|set', params)
    return res
}

const del = async (params: DelArgs): Promise<string> => {
    let res = await request<string>('plugin:redis|del', params)
    return res
}

const get = async (params: GetArgs): Promise<any> => {
    let res = await request<any>('plugin:redis|get', params)
    return res
}

const expire = async (params: ExpireArgs): Promise<any> => {
    let res = await request<any>('plugin:redis|expire', params)
    return res
}

export {
    info, keys, set, del, get, expire,
    reset,  // string
    lpush, rpush, lpop, rpop, lset,  // list
    sadd, srem,  // set
    zadd, zrem,  // zset
    hset, hdel,  // hash
    json_set  // json
}

