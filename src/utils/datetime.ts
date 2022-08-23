const diffDatetime = async (before: Date, after: Date | null = null): Promise<string> => {
    if (!after) {
        after = new Date()
    }
    let diff = after.getTime() - before.getTime()
    if (diff < 0) {
        return 'Error'
    }
    if (diff > 24 * 3600 * 1000) {
        return 'll ago'
    } else if (diff > 3600 * 1000) {
        return '> 1h'
    } else if (diff > 600 * 1000) {
        return '> 10m'
    } else if (diff > 60 * 1000) {
        return `${Math.floor(diff / 60 / 1000)}m`
    } else {
        return `less 1m`
    }
}

export { diffDatetime }
