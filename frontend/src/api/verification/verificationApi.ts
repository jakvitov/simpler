import axios from "axios";
import type {MpsVerificationRequest, MpsVerificationResponse} from "./verificationTypes.ts";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const verifyMpsCall = async (request: MpsVerificationRequest): Promise<MpsVerificationResponse> => {
    const response = await axios.post<MpsVerificationResponse>(
        `${API_BASE_URL}/api/simpler/mps/verify`,
        request,
    );
    return response.data;
};