import type {SolveLpErrorResponse, SolveLpRequest} from "../solveLpTypes.ts";
import axios, {type AxiosError} from "axios";
import type {SolveLpTwoPhaseSimplexResponseDto} from "./twoPhaseSimplexSolveTypes.ts";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchSolveTwoPhaseSimplex = async (request: SolveLpRequest): Promise<SolveLpTwoPhaseSimplexResponseDto|SolveLpErrorResponse> => {
    try {
        const response = await axios.post<SolveLpTwoPhaseSimplexResponseDto> (
            `${API_BASE_URL}/be/simpler/solve-lp/two-phase`,
            request
        );

        return response.data as SolveLpTwoPhaseSimplexResponseDto;
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