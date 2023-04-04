import { Dispatch } from 'react';
import { useDispatch } from 'react-redux';
const useDatabendDispatch = <T>()=> {
  return useDispatch<Dispatch<T>>();
};
export default useDatabendDispatch;