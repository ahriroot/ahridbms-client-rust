import BigIntVue from '@/components/postgres/cells/BigInt.vue'
import VarCharVue from '@/components/postgres/cells/VarChar.vue'
import BoolVue from '@/components/postgres/cells/Bool.vue'
import TimestampVue from '@/components/postgres/cells/Timestamp.vue'
import FloatVue from '@/components/postgres/cells/Float.vue'


const FieldComponents: { [key: string]: any } = {
    int: {
        width: 160,
        component: BigIntVue
    },
    int2: {
        width: 120,
        component: BigIntVue
    },
    int4: {
        width: 120,
        component: BigIntVue
    },
    int8: {
        width: 160,
        component: BigIntVue
    },
    smallint: {
        width: 160,
        component: BigIntVue
    },
    integer: {
        width: 160,
        component: BigIntVue
    },
    bigint: {
        width: 160,
        component: BigIntVue
    },
    decimal: {
        width: 160,
        component: BigIntVue
    },
    numeric: {
        width: 160,
        component: BigIntVue
    },
    serial: {
        width: 160,
        component: BigIntVue
    },
    serial2: {
        width: 160,
        component: BigIntVue
    },
    serial4: {
        width: 160,
        component: BigIntVue
    },
    serial8: {
        width: 160,
        component: BigIntVue
    },
    float: {
        width: 160,
        component: FloatVue
    },
    float2: {
        width: 160,
        component: FloatVue
    },
    float4: {
        width: 160,
        component: FloatVue
    },
    float8: {
        width: 160,
        component: FloatVue
    },
    money: {
        width: 160,
        component: FloatVue
    },
    char: {
        width: undefined,
        component: VarCharVue
    },
    varchar: {
        width: undefined,
        component: VarCharVue
    },
    text: {
        width: undefined,
        component: VarCharVue
    },
    name: {
        width: 200,
        component: VarCharVue
    },
    bool: {
        width: 90,
        component: BoolVue
    },
    time: {
        width: 200,
        component: TimestampVue
    },
    timetz: {
        width: 200,
        component: TimestampVue
    },
    timestamp: {
        width: 200,
        component: TimestampVue
    },
    timestamptz: {
        width: 200,
        component: TimestampVue
    },
    date: {
        width: 200,
        component: TimestampVue
    },
    interval: {
        width: 200,
        component: TimestampVue
    },
    point: {
        width: undefined,
        component: VarCharVue
    },
    line: {
        width: undefined,
        component: VarCharVue
    },
    lseg: {
        width: undefined,
        component: VarCharVue
    },
    box: {
        width: undefined,
        component: VarCharVue
    },
    path: {
        width: undefined,
        component: VarCharVue
    },
    polygon: {
        width: undefined,
        component: VarCharVue
    },
    circle: {
        width: undefined,
        component: VarCharVue
    },
    cidr: {
        width: undefined,
        component: VarCharVue
    },
    inet: {
        width: undefined,
        component: VarCharVue
    },
    macaddr: {
        width: undefined,
        component: VarCharVue
    }
}

export { FieldComponents }