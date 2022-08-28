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
