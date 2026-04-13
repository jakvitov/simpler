import type {SolveLpErrorResponse, SolveLpRequest} from "../solveLpTypes.ts";
import axios, {type AxiosError} from "axios";
import type {SolveLpRevisedSimlexResponseDto} from "./revisedSimplexSolveTypes.ts";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchSolveRevisedSimplex = async (request: SolveLpRequest): Promise<SolveLpRevisedSimlexResponseDto|SolveLpErrorResponse> => {
    try {
        const response = await axios.post<SolveLpRevisedSimlexResponseDto> (
            `${API_BASE_URL}/be/simpler/solve-lp/revised`,
            request
        );

        return response.data as SolveLpRevisedSimlexResponseDto;
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