import type BasicSimplexSolveResponse from "./basicSimplexSolveTypes.ts";
import axios, {type AxiosError} from "axios";
import type {SolveLpErrorResponse, SolveLpRequest} from "../solveLpTypes.ts";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchSolveBasicSimplex = async (request: SolveLpRequest): Promise<BasicSimplexSolveResponse|SolveLpErrorResponse> => {
    try {
        const response = await axios.post<BasicSimplexSolveResponse> (
        `${API_BASE_URL}/be/simpler/solve-lp/basic`,
        request
        );

        console.log(typeof (response.data as BasicSimplexSolveResponse));
        return response.data as BasicSimplexSolveResponse;
    } catch (error) {
        try {
            const err = error as AxiosError;
            if (err.response?.status === 400) {
                return err.response.data as SolveLpErrorResponse;
            }
        } catch (e) {
            alert("Error occurred: " + error);
        }
        throw error;
    }

}
