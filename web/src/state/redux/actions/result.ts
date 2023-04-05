import { tResultsItem } from '@/types';
import { DO_FETCHING, SET_PRE_QUESTION, SHOW_ERROR_TIP, UPDATE_RESULT } from '../constant/result';

export interface IGetResultsAction {
  type: UPDATE_RESULT;
  payload: tResultsItem;
}

export interface IDoFetchingAction {
  type: DO_FETCHING;
  payload: boolean;
}

export interface ISetPreQuestionAction {
  type: SET_PRE_QUESTION;
  payload: string;
}

export interface IShowErrorTipAction {
  type: SHOW_ERROR_TIP;
  payload: boolean;
}
export type ResultsActions =
| IGetResultsAction
| IDoFetchingAction
| ISetPreQuestionAction
| IShowErrorTipAction