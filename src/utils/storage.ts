import { Connection } from '@/types/Connection';
import { OpenTabMesagae } from '@/types/Message';

const saveConnections = async (connections: Connection[]) => {
    localStorage.setItem('connections', JSON.stringify(connections))
}

const getConnections = async (): Promise<Connection[]> => {
    const connections = localStorage.getItem('connections')
    return connections ? JSON.parse(connections) : []
}

const saveTabs = async (connections: OpenTabMesagae[]) => {
    localStorage.setItem('tabs', JSON.stringify(connections))
}

const getTabs = async (): Promise<OpenTabMesagae[]> => {
    const tabs = localStorage.getItem('tabs')
    return tabs ? JSON.parse(tabs) : []
}

export { saveConnections, getConnections, saveTabs, getTabs }
