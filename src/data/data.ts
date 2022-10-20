import MongodbConnect from '@/components/mongodb/Connection.vue'
import MongodbCollection from '@/components/mongodb/Collection.vue'

import PostgresConnect from '@/components/postgres/Connection.vue'
import PostgresTableVue from '@/components/postgres/Table.vue'
import PostgresQueryVue from '@/components/postgres/Query.vue'
import PostgresCreateTableVue from '@/components/postgres/CreateTable.vue'
import PostgresCreateTableSVue from '@/components/postgres/CreateTableS.vue'

import RedisConnect from '@/components/redis/Connection.vue'
import RedisTabVue from '@/components/redis/TabDb.vue'
import RedisQueryVue from '@/components/redis/TabQuery.vue'
import RedisInfoVue from '@/components/redis/Info.vue'

import iconPostgres from '@/components/icon/postgres.vue'
import iconRedis from '@/components/icon/redis.vue'
import iconMongodb from '@/components/icon/mongodb.vue'

const DBType = [
    {
        label: 'Redis',
        value: 'redis',
        icon: iconRedis
    },
    {
        label: 'Postgres',
        value: 'postgres',
        icon: iconPostgres
    },
    {
        label: 'Mongodb',
        value: 'mongodb',
        icon: iconMongodb
    }
]

const ConnectionComponents = {
    'redis': RedisConnect,
    'postgres': PostgresConnect,
    'mongodb': MongodbConnect,
}

const TabComponents = {
    'redis:db': RedisTabVue,
    'redis:query': RedisQueryVue,
    'postgres:table': PostgresTableVue,
    'postgres:query': PostgresQueryVue,
    'postgres:create_table': PostgresCreateTableVue,
    'postgres:create_table_s': PostgresCreateTableSVue,
    'mongodb:collection': MongodbCollection,
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

const MongodbConnectInit = {
    name: '',
    host: 'localhost',
    port: '27017',
    user: '',
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
    DBType, ConnectionComponents, TabComponents, InfoComponents, RedisConnectInit, PostgresConnectInit, MongodbConnectInit,
    QuerySuggestionsOfRedis
}