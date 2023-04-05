import { DO_FETCHING, SET_INPUT_QUESTION, SET_PRE_QUESTION, SHOW_ERROR_TIP, UPDATE_RESULT } from '../constant/result';
import { ResultsActions } from '../actions/result';
import useDatabendDispatch from '@/state/hooks/useDatabendDispatch';
import { tResultsItem } from '@/types';

const useResultsDispatch = () => {
  const resultDispatch = useDatabendDispatch<ResultsActions>();

  function dispatchUpdateResultList(payload: tResultsItem) {
    dispatch(UPDATE_RESULT, payload);
  }
  function dispatchIsFetching(payload: boolean) {
    dispatch(DO_FETCHING, payload);
  }

  function dispatchSetPreQuestion(payload: string) {
    dispatch(SET_PRE_QUESTION, payload);
  }

  function dispatchSetInputQuestion(payload: string) {
    dispatch(SET_INPUT_QUESTION, payload);
  }
  function dispatchShowErrorTip(payload: boolean) {
    dispatch(SHOW_ERROR_TIP, payload);
  }

  
  function dispatch(type: any, payload: any) {
    resultDispatch({type, payload});
  }
  return {
    dispatchUpdateResultList,
    dispatchIsFetching,
    dispatchSetPreQuestion,
    dispatchShowErrorTip,
    dispatchSetInputQuestion
  };
};


export default useResultsDispatch;
