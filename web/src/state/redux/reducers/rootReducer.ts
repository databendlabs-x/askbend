import { combineReducers } from 'redux';
import resultReducer from './result';
const rootReducer = combineReducers({
  results: resultReducer
});
export type AppState = ReturnType<typeof rootReducer>;
export default rootReducer;