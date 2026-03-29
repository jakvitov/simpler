import axios from "axios";
import type {LastErrorResponseDto} from "./lastErrorTypes.ts";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchLastErrorData = async (): Promise<LastErrorResponseDto|null> => {
    try {
        const response = await axios.get<LastErrorResponseDto> (
            `${API_BASE_URL}/be/simpler/error/last`,
        );

        return response.data as LastErrorResponseDto;
    } catch (error) {
        return null;
        throw error;
    }

}