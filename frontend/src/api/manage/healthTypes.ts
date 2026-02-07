export type HelathResponseStatus =
    | "OK"
    | "ERROR"


export interface HealthResponse {
    status: HelathResponseStatus
    version: string
}
