import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { Response } from '@/types/redis'
import { Connection } from "@/types/Connection"
import { PostgresConnect } from "@/types/postgres"
import { isReactive } from "vue"


interface GetDatabasesArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
}

interface GetTablesArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
}

interface GetColumnsArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
    table: string
}

interface GetPrimaryKeysArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
    table: string
}

interface GetTableStructArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
    table: string
}

interface SelectArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    skip: number
    limit: number
    page: number
    size: number
    sorts: {
        field: string
        order: string
    }[]
    database: string
    table: string
}

interface UpdateArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
    sql: string
}

interface ExecuteWithTransactionArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
    sqls: string[]
}

interface ExecuteSelectSqlArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
    sql: string
}

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
        return Promise.resolve({ is_error: true, code: res.code, msg: res.msg, from: '' } as any)
    }
    return res.data.Success
}

const getDatabases = async (params: GetDatabasesArgs): Promise<any> => {
    let res = await request<any>('plugin:postgres|get_databases', params)
    return res
}

const getTables = async (params: GetTablesArgs): Promise<any> => {
    let res = await request<any>('plugin:postgres|get_tables', params)
    return res
}

const getColumns = async (params: GetColumnsArgs): Promise<any> => {
    let res = await request<any>('plugin:postgres|get_columns', params)
    return res
}

const getPrimaryKeys = async (params: GetPrimaryKeysArgs): Promise<any> => {
    let res = await request<any>('plugin:postgres|get_primary_keys', params)
    return res
}

const getTableStruct = async (params: GetTableStructArgs): Promise<any[]> => {
    let res = await request<any[]>('plugin:postgres|get_table_struct', params)
    let result: any[] = []
    res.forEach((rowData: any[]) => {
        let row: any = {}
        rowData.forEach((column: any) => {
            for (let k in column) {
                row[column[k].key] = column[k].value
            }
        })
        result.push(row)
    })
    return result
}

const select = async (params: SelectArgs): Promise<{
    data: any[]
    count: number
}> => {
    let res = await request<{
        data: any[]
        count: number
    }>('plugin:postgres|select', params)
    let result: any[] = []
    res.data.forEach((rowData: any[]) => {
        let row: any[] = []
        rowData.forEach((column: any) => {
            for (let k in column) {
                row.push({
                    type: k,
                    field: column[k].key,
                    value: column[k].value,
                    old: column[k].value
                })
            }
        })
        result.push(row)
    })
    return {
        data: result,
        count: res.count
    }
}

const update = async (params: UpdateArgs): Promise<any> => {
    let res = await request<any>('plugin:postgres|update', params)
    return res
}

const executeWithTransaction = async (params: ExecuteWithTransactionArgs): Promise<any> => {
    let res = await request<any>('plugin:postgres|execute_with_transaction', params)
    return res
}

const executeSelectSql = async (params: ExecuteSelectSqlArgs, analysis: boolean = true): Promise<any> => {
    let res = await request<any>('plugin:postgres|execute_select_sql', params)
    if (analysis) {
        let result: any[] = []
        res.forEach((rowData: any[]) => {
            let row: any = {}
            rowData.forEach((column: any) => {
                for (let k in column) {
                    row[column[k].key] = column[k].value
                }
            })
            result.push(row)
        })
        return result
    } else {
        return res
    }
}

export { getDatabases, getTables, getColumns, getPrimaryKeys, getTableStruct, select, update, executeWithTransaction, executeSelectSql }

