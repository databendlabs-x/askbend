import { combineReducers } from 'redux';
import resultReducer from './results';
const rootReducer = combineReducers({
  results: resultReducer
});
export type AppState = ReturnType<typeof rootReducer>;
export default rootReducer;