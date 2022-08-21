import RedisConnect from '@/components/redis/Connection.vue'

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
}

const RedisConnectInit = {
    name: '',
    host: 'localhost',
    port: '6379',
    user: '',
    pass: '',
    index: '0'
}

export { DBType, ConnectionComponents, RedisConnectInit }