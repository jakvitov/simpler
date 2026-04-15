import axios from 'axios';
import type {HealthResponse} from './healthTypes.ts';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchHealthCheck = async (): Promise<HealthResponse> => {
    const response = await axios.get<HealthResponse>(
        `${API_BASE_URL}/api/simpler/health`,
        {},
    );

    return response.data;
};