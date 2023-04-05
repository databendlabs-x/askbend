
import { useSelector } from 'react-redux';
import { AppState } from '../redux/reducers/rootReducer';
export const useGetResultsState = () => {
  return useSelector((state: AppState) => state.results);
};
