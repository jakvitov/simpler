import axios from 'axios';
import type {HealthResponse} from './healthTypes.ts';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export const fetchHealthCheck = async (): Promise<HealthResponse> => {
    const response = await axios.post<HealthResponse>(
        `${API_BASE_URL}/be/simpler/health`,
        {},
        {
            timeout: 5000,
            headers: {
                'Content-Type': 'application/json',
            }
        }
    );

    return response.data;
};