
import { useSelector } from 'react-redux';
import { AppState } from '../redux/reducers/rootReducer';
import { ResultsState } from '../redux/reducers/results';
export const useGetResultsState = () => {
  return useSelector((state: AppState) => state.results as ResultsState);
};
