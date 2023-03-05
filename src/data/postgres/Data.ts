import BigIntVue from '@/components/postgres/cells/BigInt.vue'
import VarCharVue from '@/components/postgres/cells/VarChar.vue'
import BoolVue from '@/components/postgres/cells/Bool.vue'
import TimestampVue from '@/components/postgres/cells/Timestamp.vue'
import FloatVue from '@/components/postgres/cells/Float.vue'


const FieldComponents: { [key: string]: any } = {
    int: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    int2: {
        width: 120,
        minWidth: 120,
        component: BigIntVue
    },
    int4: {
        width: 120,
        minWidth: 120,
        component: BigIntVue
    },
    int8: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    smallint: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    integer: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    bigint: {
        width: 160,
        component: BigIntVue
    },
    decimal: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    numeric: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    serial: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    serial2: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    serial4: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    serial8: {
        width: 160,
        minWidth: 160,
        component: BigIntVue
    },
    float: {
        width: 160,
        minWidth: 160,
        component: FloatVue
    },
    float2: {
        width: 160,
        minWidth: 160,
        component: FloatVue
    },
    float4: {
        width: 160,
        minWidth: 160,
        component: FloatVue
    },
    float8: {
        width: 160,
        minWidth: 160,
        component: FloatVue
    },
    money: {
        width: 160,
        minWidth: 160,
        component: FloatVue
    },
    char: {
        width: undefined,
        minWidth: 160,
        component: VarCharVue
    },
    varchar: {
        width: undefined,
        minWidth: 160,
        component: VarCharVue
    },
    text: {
        width: undefined,
        minWidth: 160,
        component: VarCharVue
    },
    name: {
        width: 200,
        minWidth: 160,
        component: VarCharVue
    },
    bool: {
        width: 90,
        minWidth: 90,
        component: BoolVue
    },
    timetz: {
        width: 200,
        minWidth: 200,
        component: TimestampVue
    },
    timestamp: {
        width: 200,
        minWidth: 200,
        component: TimestampVue
    },
    timestamptz: {
        width: 200,
        minWidth: 200,
        component: TimestampVue
    },
    interval: {
        width: 200,
        minWidth: 200,
        component: TimestampVue
    },
    point: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    line: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    lseg: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    box: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    path: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    polygon: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    circle: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    cidr: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    inet: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    macaddr: {
        width: undefined,
        minWidth: 100,
        component: VarCharVue
    },
    date: {
        width: undefined,
        minWidth: 200,
        component: VarCharVue
    },
    time: {
        width: undefined,
        minWidth: 200,
        component: VarCharVue
    }
}

export { FieldComponents }