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

export { DBType, ConnectionComponents, TabComponents, RedisConnectInit, PostgresConnectInit }