// Copyright 2023 Datafuse Labs.
import React, { useEffect, useState } from 'react';
import { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import DatabendSvg from '../../assets/svg/databend';
import SendSvg from '../../assets/svg/send';
import axios from 'axios';
import { useSafeState } from 'ahooks';
import useResultsDispatch from '../../state/redux/dispatch/results';
import WhiteLoading from '../Loading';
import { KEY_CODE } from '../../assets/constant';
import { useGetResultsState } from '../../state/hooks/useGetResultsState';
const QuestionInput: FC = (): ReactElement=> {
  const INPUT_ID = 'question-input';
  const { dispatchUpdateResultList, dispatchIsFetching } =  useResultsDispatch();
  const [queryText, setQueryText] = useSafeState('');
  const { isFeatching } = useGetResultsState();
  const [isRegenerate, setIsRegenerate] = useState(false);
  useEffect(()=>{
    window.addEventListener('keydown', onKeyDown);
    return ()=>{
      window.removeEventListener('keydown', onKeyDown);
    };
  }, [queryText]);
  function onKeyDown(e: React.KeyboardEvent<HTMLInputElement> | any){
    const code = e.keyCode || e.which;
    if ((e.target as HTMLInputElement)?.id === INPUT_ID && code === KEY_CODE.ENTER) {
      getResults();
    }
  }
  async function getResults(isRegenerate = false) {
    dispatchIsFetching(true);
    try{
      const data = await axios.post('/query', {
        query: queryText
      });
      if ([200, 201]?.includes(data?.status )) {
        const res = data?.data?.result;
        dispatchUpdateResultList({value: res, isRegenerate});
        setIsRegenerate(true);
      }
    } finally{
      dispatchIsFetching(false);
    }
  }
  function changeQueryText(e: React.ChangeEvent<HTMLInputElement>) {
    setQueryText(e?.target?.value);
    setIsRegenerate(false);
  }
  return (
    <span className={styles.wrap}>
      <span className={styles.prefix}>
        <DatabendSvg></DatabendSvg>
      </span>
      <input 
        id={INPUT_ID}
        onChange={(e)=> changeQueryText(e)}
        placeholder='Ask AI to question?' className={styles.input} type='text'></input>
      <span className={styles.suffix}>
        {
          isFeatching ? (
            <span className={styles.send}><WhiteLoading /></span>
          ) : (
            isRegenerate ? (
              <span 
                onClick={() => getResults(true)}
                className={styles.regenerate}>
                  Regenerate
              </span>
            ) : (
              queryText ? (
                <span 
                  onClick={() => getResults()}
                  className={styles.send}>
                  <SendSvg />
                </span>
              ) : (
                <span className={styles.text}>AI</span>
              )
            )
          )
        }
      </span>
    </span>
  );
};
export default QuestionInput;