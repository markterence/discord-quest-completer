export function safeParseJSON<T>(jsonString: unknown, defaultValue: T): T{
    try {
        if (typeof jsonString === 'string') {
            return JSON.parse(jsonString) as T;
        }
        if (typeof jsonString === 'object' && jsonString !== null) {
            return jsonString as T;
        }
        if (Array.isArray(jsonString) || jsonString instanceof Object) {
            return jsonString as T;
        }
    } catch (error) {
        return defaultValue;
    }
}