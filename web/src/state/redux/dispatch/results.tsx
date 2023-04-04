import { DO_FETCHING, UPDATE_RESULT } from '../constant/results';
import { ResultsActions } from '../actions/results';
import useDatabendDispatch from '../../hooks/useDatabendDispatch';
import { tResultsItem } from '../../../types';

const useResultsDispatch = () => {
  const resultDispatch = useDatabendDispatch<ResultsActions>();

  function dispatchUpdateResultList(payload: tResultsItem) {

    dispatch(UPDATE_RESULT, payload);
  }
  function dispatchIsFetching(payload: boolean) {

    dispatch(DO_FETCHING, payload);
  }

  function dispatch(type: any, payload: any) {
    resultDispatch({type, payload});
  }
  return {
    dispatchUpdateResultList,
    dispatchIsFetching
  };
};


export default useResultsDispatch;
