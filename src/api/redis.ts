import { InvokeArgs } from "@tauri-apps/api/tauri"
import { AllKeys } from '@/types/redis'
import { reset } from "./resis/string"
import { lpush, rpush, lpop, rpop, lset } from "./resis/list"
import { sadd, srem } from "./resis/set"
import { zadd, zrem } from "./resis/zset"
import { hset, hdel } from "./resis/hash"
import { json_set } from "./resis/json"
import request from "./resis/request"
import { IExecResult } from "@/types/redis/Data"


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

const del = async (params: DelArgs): Promise<number> => {
    let res = await request<number>('plugin:redis|del', params)
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

interface ExecArgs extends InvokeArgs {
    conn: any
    commandLines: string[]
    db: string
}

const exec = async (params: ExecArgs): Promise<IExecResult[]> => {
    let res = await request<IExecResult[]>('plugin:redis|exec', params)
    return res
}

interface TestArgs extends InvokeArgs {
    conn: any
    db: string
}

const test = async (params: TestArgs): Promise<IExecResult[]> => {
    let res = await request<any>('plugin:redis|test', params)
    return res
}

export {
    info, keys, set, del, get, expire,
    reset,  // string
    lpush, rpush, lpop, rpop, lset,  // list
    sadd, srem,  // set
    zadd, zrem,  // zset
    hset, hdel,  // hash
    json_set,  // json
    exec,
    test
}

