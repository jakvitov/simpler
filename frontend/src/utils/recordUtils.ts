
export function recordToMap<A extends string | number | symbol, B>(record: Record<A, B>): Map<A, B> {
    return new Map(Object.entries(record) as [A, B][]);
}