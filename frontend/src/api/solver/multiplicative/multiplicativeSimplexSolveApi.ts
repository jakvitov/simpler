import type {SolveLpErrorResponse, SolveLpRequest} from "../solveLpTypes.ts";
import axios, {type AxiosError} from "axios";
import type {SolveLpMultiplicativeSimplexResponseDto} from "./multiplicativeSimplexSolveTypes.ts";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchSolveMultiplicativeSimplex = async (request: SolveLpRequest): Promise<SolveLpMultiplicativeSimplexResponseDto|SolveLpErrorResponse> => {
    try {
        const response = await axios.post<SolveLpMultiplicativeSimplexResponseDto> (
            `${API_BASE_URL}/api/simpler/solve-lp/multiplicative`,
            request
        );

        return response.data as SolveLpMultiplicativeSimplexResponseDto;
    } catch (error) {
        try {
            const err = error as AxiosError;
            if (err.response?.status === 400 || err.response?.status === 500) {
                return err.response.data as SolveLpErrorResponse;
            }
        } catch (e) {
            alert("Error occurred: " + error);
        }
        throw error;
    }

}