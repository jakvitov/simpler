export interface MpsVerificationRequest {
    data: string
}

export type MpsVerificationResponseStatus =
    | "OK"
    | "VERIFICATION_FAILED";


export interface MpsVerificationResponse {
    status: MpsVerificationResponseStatus,
    errors: string[]
}