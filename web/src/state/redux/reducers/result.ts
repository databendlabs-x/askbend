
import { tResultsItem } from '@/types';
import { ResultsActions } from '../actions/result';
import { DO_FETCHING, SET_INPUT_QUESTION, SET_PRE_QUESTION, SHOW_ERROR_TIP, UPDATE_RESULT } from '../constant/result';
export type ResultsState = {
  resultsList: tResultsItem[];
  isFeatching: boolean | undefined;
  preQuestion: string;
  inputQuestion: string;
  showErrorTip: boolean;
}
const initialState: ResultsState = {
  resultsList: [],
  isFeatching: undefined,
  preQuestion: '',
  inputQuestion: '',
  showErrorTip: false
};
const resultsReducer = (state: ResultsState = initialState, action: ResultsActions) => {
  switch(action.type) {
  case UPDATE_RESULT:
  {
    const { resultsList } = state;
    if (!action.payload?.isRegenerate) {
      resultsList.splice(0);
    }
    resultsList?.unshift({
      ...action.payload,
      date: Date.now()
    });
    return {
      ...state,
      resultsList: resultsList
    };
  }
  case DO_FETCHING:
    return {
      ...state,
      isFeatching: action.payload
    };
  case SET_PRE_QUESTION: 
    return {
      ...state,
      preQuestion: action.payload
    };
  case SET_INPUT_QUESTION: 
    return {
      ...state,
      inputQuestion: action.payload
    };
  case SHOW_ERROR_TIP: 
    return {
      ...state,
      showErrorTip: action.payload
    };
  default:
    return state;
  }
};
export default resultsReducer;