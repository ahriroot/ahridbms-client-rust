import RedisConnect from '@/components/redis/Connection.vue'
import PostgresConnect from '@/components/postgres/Connection.vue'

import RedisTabVue from '@/components/redis/TabDb.vue'
import RedisQueryVue from '@/components/redis/TabQuery.vue'

const DBType = [
    {
        label: 'Redis',
        value: 'redis'
    },
    {
        label: 'Postgres',
        value: 'postgres'
    }
]

const ConnectionComponents = {
    'redis': RedisConnect,
    'postgres': PostgresConnect,
}

const TabComponents = {
    'redis:db': RedisTabVue,
    'redis:query': RedisQueryVue,
}

const RedisConnectInit = {
    name: '',
    host: 'localhost',
    port: '6379',
    user: '',
    pass: '',
    index: '0'
}

const PostgresConnectInit = {
    name: '',
    host: 'localhost',
    port: '5432',
    user: 'postgres',
    pass: '',
    db: ''
}

const QuerySuggestionsOfRedis = async (kind: any, snippet: any) => ([
    {
        label: "GET",
        kind: kind.Function,
        insertText: "GET ${0:key}",
        insertTextRules: snippet,
        detail: "Get the value of a key",
    },
    {
        label: "JSON.GET",
        kind: kind.Function,
        insertText: "GET ${0:key}",
        insertTextRules: snippet,
        detail: "Get Json data",
    },
    {
        label: "JSON.SET",
        kind: kind.Function,
        insertText: "GET ${1:key} $ ${0:json_str}",
        insertTextRules: snippet,
        detail: "SET Json data",
    },
])

export {
    DBType, ConnectionComponents, TabComponents, RedisConnectInit, PostgresConnectInit,
    QuerySuggestionsOfRedis
}