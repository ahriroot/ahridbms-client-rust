import PostgresConnect from '@/components/postgres/Connection.vue'
import PostgresTableVue from '@/components/postgres/Table.vue'

import RedisConnect from '@/components/redis/Connection.vue'
import RedisTabVue from '@/components/redis/TabDb.vue'
import RedisQueryVue from '@/components/redis/TabQuery.vue'
import RedisInfoVue from '@/components/redis/Info.vue'

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
    'postgres:table': PostgresTableVue,
}

const InfoComponents = {
    'redis': RedisInfoVue
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
        detail: "Get string data",
    },
    {
        label: "SET",
        kind: kind.Function,
        insertText: "SET ${1:key} ${0:value}",
        insertTextRules: snippet,
        detail: "SET string data",
    },
    {
        label: "JSON.GET",
        kind: kind.Function,
        insertText: "JSON.GET ${0:key}",
        insertTextRules: snippet,
        detail: "Get Json data",
    },
    {
        label: "JSON.SET",
        kind: kind.Function,
        insertText: "JSON.SET ${1:key} $ ${0:json_str}",
        insertTextRules: snippet,
        detail: "SET Json data",
    },
])

export {
    DBType, ConnectionComponents, TabComponents, InfoComponents, RedisConnectInit, PostgresConnectInit,
    QuerySuggestionsOfRedis
}