import { IExecType } from "@/types/redis/Data"

const NewFieldValue = {
    string: {
        key: '',
        value: '',
        ttl: '-1'
    },
    list: {
        key: '',
        value: [
            {
                value: ''
            }
        ],
        ttl: '-1'
    },
    set: {
        key: '',
        value: [
            {
                value: ''
            }
        ],
        ttl: '-1'
    },
    zset: {
        key: '',
        value: [
            {
                score: 0,
                value: ''
            }
        ],
        ttl: '-1'
    },
    hash: {
        key: '',
        value: [
            {
                field: '',
                value: ''
            }
        ],
        ttl: '-1'
    },
}

const ExecType: IExecType = {
    nil: 'Nil',
    okay: 'Okay',
    data: 'Data',
    status: 'Status',
    integer: 'Integer',
    bulk: 'Bulk',
    error: 'Error'
}

export { NewFieldValue, ExecType }