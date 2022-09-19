import { Connection } from '@/types/Connection';
import { OpenTabMesagae } from '@/types/Message';
import { RedisConnect } from '@/types/redis';

const saveConnections = async (connections: Connection<RedisConnect>[]) => {
    localStorage.setItem('connections', JSON.stringify(connections))
}

const getConnections = async (): Promise<Connection<RedisConnect>[]> => {
    const connections = localStorage.getItem('connections')
    return connections ? JSON.parse(connections) : []
}

const saveTabs = async (connections: OpenTabMesagae<any>[]) => {
    localStorage.setItem('tabs', JSON.stringify(connections))
}

const getTabs = async (): Promise<OpenTabMesagae<any>[]> => {
    const tabs = localStorage.getItem('tabs')
    return tabs ? JSON.parse(tabs) : []
}

export { saveConnections, getConnections, saveTabs, getTabs }
