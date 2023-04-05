
import { useSelector } from 'react-redux';
import { AppState } from '@/state/redux/reducers/rootReducer';
import { ResultsState } from '@/state/redux/reducers/results';
export const useGetResultsState = () => {
  return useSelector((state: AppState) => state.results as ResultsState);
};
