import type BasicSimplexSolveResponse from "./basicSimplexSolveTypes.ts";
import axios from "axios";
import type SolveLpRequest from "../solveLpTypes.ts";

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchSolveBasicSimplex = async (request: SolveLpRequest): Promise<BasicSimplexSolveResponse> => {
    const response = await axios.post<BasicSimplexSolveResponse> (
        `${API_BASE_URL}/be/simpler/solve-lp/basic`,
        request
    );
    return response.data;
}
