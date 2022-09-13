export interface PgDatabase {
    oid: number
    datname: string
    datdba: number
    encoding: number
    datcollate: string
    datctype: string
    datistemplate: boolean
    datallowconn: boolean
    datconnlimit: number
    datlastsysoid: number
    datfrozenxid?: null
    datminmxid?: null
    dattablespace: number
    datacl?: null
}

export interface PgTable {
    schemaname: string
    tablename: string
    tableowner: string
    tablespace: any
    hasindexes: boolean | null
    hasrules: boolean | null
    hastriggers: boolean | null
    rowsecurity: boolean | null
}

export interface PgColumn {
    attnum: number
    field: string
    type: string
    length: number
    lengthvar: number
    notnull: boolean
}