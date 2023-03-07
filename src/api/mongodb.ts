import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { Connection } from "@/types/Connection"
import { MongodbConnect } from "@/types/mongodb"
import { Response } from "@/types/mongodb/Data"


const request = async <T>(command: string, params: any): Promise<T> => {
    let res = await invoke<Response<T>>(command, params)
    if (res.code !== 10000) {
        if (res.code < 50000) {
            window.$message.warning(res.msg, {
                closable: true,
                duration: 10000
            })
        } else {
            window.$message.error(res.msg)
        }
        throw new Error(res.msg)
    }
    return res.data.Success
}

interface DatabaseArgs extends InvokeArgs {
    conn: Connection<MongodbConnect>
    database?: string
}

const databases = async (params: DatabaseArgs): Promise<any> => {
    params.database = ""
    let res = await request<any>('plugin:mongodb|databases', params)
    return res
}

interface CollectionArgs extends InvokeArgs {
    conn: Connection<MongodbConnect>
    database: string
}

const collections = async (params: CollectionArgs): Promise<any> => {
    let res = await request<any>('plugin:mongodb|collections', params)
    return res
}

interface DocumentArgs extends InvokeArgs {
    conn: Connection<MongodbConnect>
    database: string
    collection: string
    skip: number
    limit: number
    page: number
    size: number
    sorts: {
        field: string
        order: string
    }[]
}

const documents = async (params: DocumentArgs): Promise<any> => {
    let res = await request<any>('plugin:mongodb|documents', params)
    return res
}

interface DeleteManyArgs extends InvokeArgs {
    conn: Connection<MongodbConnect>
    database: string
    collection: string
    documents: object[]
}

const deleteMany = async (params: DeleteManyArgs): Promise<any> => {
    let res = await request<any>('plugin:mongodb|delete_many', params)
    return res
}

interface DropDatabaseManyArgs extends InvokeArgs {
    conn: Connection<MongodbConnect>
    database: string
}

const dropDatabase = async (params: DropDatabaseManyArgs): Promise<any> => {
    let res = await request<any>('plugin:mongodb|drop_database', params)
    return res
}

interface DropCollectionArgs extends InvokeArgs {
    conn: Connection<MongodbConnect>
    database: string
    collection: string
}

const dropCollection = async (params: DropCollectionArgs): Promise<any> => {
    let res = await request<any>('plugin:mongodb|drop_collection', params)
    return res
}

interface TestArgs extends InvokeArgs {
    conn: Connection<MongodbConnect>
    database: string
}

const test = async (params: TestArgs): Promise<any> => {
    let res = await request<any>('plugin:mongodb|test', params)
    return res
}

export { databases, collections, documents, deleteMany, dropDatabase, dropCollection, test }

