import { Connection } from '@/types/Connection';

const saveConnections = async (connections: Connection[]) => {
    localStorage.setItem('connections', JSON.stringify(connections))
}

const getConnections = async (): Promise<Connection[]> => {
    const connections = localStorage.getItem('connections')
    return connections ? JSON.parse(connections) : []
}

export { saveConnections, getConnections }
