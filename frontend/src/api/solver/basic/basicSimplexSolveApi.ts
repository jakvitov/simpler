import axios, {type AxiosError} from "axios";
import type {SolveLpErrorResponse, SolveLpRequest} from "../solveLpTypes.ts";
import type {SolveLpBasicSimplexResponseDto} from "./basicSimplexSolveTypes.ts";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchSolveBasicSimplex = async (request: SolveLpRequest): Promise<SolveLpBasicSimplexResponseDto|SolveLpErrorResponse> => {
    try {
        const response = await axios.post<SolveLpBasicSimplexResponseDto> (
        `${API_BASE_URL}/api/simpler/solve-lp/basic`,
        request
        );

        return response.data as SolveLpBasicSimplexResponseDto;
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
