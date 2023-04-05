import { Dispatch } from 'react';
import { useDispatch } from 'react-redux';

const useDatabendDispatch = <T>() => {
  // eslint-disable-next-line @typescript-eslint/ban-ts-comment
  // @ts-ignore
  return useDispatch<Dispatch<T>>();
};
export default useDatabendDispatch;