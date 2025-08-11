import axios from 'axios';

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'http://localhost:3000',
});

export interface LoyalityPass {
  serialNumber: string;
  alreadyRedeemed: number;
  totalPoints: number;
  currentPoints: number;
  passHolderName: string;
  lastUsedAt?: string;
}

const getAuthHeaders = (token: string) => ({
  headers: {
    Authorization: `Bearer ${token}`,
  },
});

export const getLoyalityPass = (serialNumber: string, token: string) => {
  return apiClient.get<LoyalityPass>(`/passes/${serialNumber}/loyality`, getAuthHeaders(token));
};

export const addPoints = (serialNumber: string, addPoints: number, token: string) => {
  return apiClient.post(`/passes/${serialNumber}/loyality/points`, { addPoints }, getAuthHeaders(token));
};

export const redeemBonus = (serialNumber: string, token: string) => {
  return apiClient.post(`/passes/${serialNumber}/loyality/bonus`, {}, getAuthHeaders(token));
};

export const createPass = (token: string) => {
  return apiClient.get('/passes', { ...getAuthHeaders(token), responseType: 'blob' });
};
