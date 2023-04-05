import axios from 'axios';
const { VITE_APP_API_BASE_URL } = import.meta.env;
export function getAnswers(question: string) {
  return axios.post(`${VITE_APP_API_BASE_URL}/query`, {
    query: question
  });
}