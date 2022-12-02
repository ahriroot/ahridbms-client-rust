import { invoke, InvokeArgs } from "@tauri-apps/api/tauri"
import { Response } from '@/types/redis'
import { Connection } from "@/types/Connection"
import { PostgresConnect } from "@/types/postgres"


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
                    type: column[k].typename,
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

const selectWithStruct = async (params: SelectArgs): Promise<{
    table_primary: any
    table_struct: any[],
    table_data: {
        data: any[]
        count: number
    }
}> => {
    let res = await request<{
        table_primary: any
        table_struct: any[],
        table_data: {
            data: any[]
            count: number
        }
    }>('plugin:postgres|select_with_struct', params)
    let result1: any[] = []
    res.table_struct.forEach((rowData: any[]) => {
        let row: any = {}
        rowData.forEach((column: any) => {
            for (let k in column) {
                row[column[k].key] = column[k].value
            }
        })
        result1.push(row)
    })
    let result2: any[] = []
    res.table_data.data.forEach((rowData: any[]) => {
        let row: any[] = []
        rowData.forEach((column: any) => {
            for (let k in column) {
                row.push({
                    type: column[k].typename,
                    field: column[k].key,
                    value: column[k].value,
                    old: column[k].value
                })
            }
        })
        result2.push(row)
    })
    return {
        table_primary: res.table_primary,
        table_struct: result1,
        table_data: {
            data: result2,
            count: res.table_data.count
        },
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

interface TestArgs extends InvokeArgs {
    conn: Connection<PostgresConnect>
    database: string
}

const test = async (params: TestArgs): Promise<any> => {
    let res = await request<any>('plugin:postgres|test', params)
    return res
}

export { getDatabases, getTables, getColumns, getPrimaryKeys, getTableStruct, select, selectWithStruct, update, executeWithTransaction, executeSelectSql, test }

