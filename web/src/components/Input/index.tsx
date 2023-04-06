// Copyright 2023 Datafuse Labs.
import { useEffect, useState } from 'react';
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
import DatabendSvg from '@/assets/svg/databend';
import SendSvg from '@/assets/svg/send';
import { useSafeState, useUpdateEffect, useMount } from 'ahooks';
import useResultsDispatch from '@/state/redux/dispatch/result';
import WhiteLoading from 'components/Loading';
import { KEY_CODE } from '@/assets/constant';
import { useGetResultsState } from '@/state/hooks/useGetResultsState';
import useGetScrollInfo from '@/hooks/useGetScrollInfo';
import clsx from 'clsx';
import { scrollToTop } from '@/utils/tools';
import { getAnswers } from '@/api';
import { Tooltip } from 'antd';
import Examples from './examples';
const QuestionInput: FC = (): ReactElement=> {
  const INPUT_ID = 'question-input';
  const { isSwitch } =  useGetScrollInfo();
  const { dispatchUpdateResultList, dispatchIsFetching, dispatchShowErrorTip, dispatchSetInputQuestion, dispatchSetPreQuestion } =  useResultsDispatch();
  const [queryText, setQueryText] = useSafeState('');
  const { isFeatching, preQuestion } = useGetResultsState();
  const [isRegenerate, setIsRegenerate] = useState(false);
  const [openExample, setOpenExample] = useState(false);
  useEffect(()=>{
    window.addEventListener('keydown', onKeyDown);
    return ()=>{
      window.removeEventListener('keydown', onKeyDown);
    };
  }, [queryText, isFeatching, isRegenerate]);
  useMount(()=> {
    document.onclick = ()=> {
      setOpenExample(false);
    };
  });
  useUpdateEffect(()=> {
    if (preQuestion) {
      setQueryText(preQuestion);
      getResults(false, preQuestion);
    }
  }, [preQuestion]);
  function onKeyDown(e: React.KeyboardEvent<HTMLInputElement> | any){
    const code = e.keyCode || e.which;
    if ((e.target as HTMLInputElement)?.id === INPUT_ID && code === KEY_CODE.ENTER) {
      setOpenExample(false);
      if (isFeatching) return;
      getResults(isRegenerate);
    }
  }
  async function getResults(isRegenerate = false, preQuestion?: string) {
    dispatchIsFetching(true);
    dispatchShowErrorTip(false);
    setOpenExample(false);
    try{
      const data = await getAnswers(preQuestion ? preQuestion : queryText);
      if ([200, 201]?.includes(data?.status )) {
        const res = data?.data?.result;
        dispatchUpdateResultList({value: res, isRegenerate});
      } else {
        dispatchShowErrorTip(true);
      }
    }
    catch (error) {
      dispatchShowErrorTip(true);
    }
    finally{
      setIsRegenerate(true);
      scrollToTop();
      dispatchIsFetching(false);
    }
  }
  function changeQueryText(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e?.target?.value;
    setQueryText(value);
    setIsRegenerate(false);
    dispatchSetInputQuestion(value);
    setOpenExample(true);
    if (!value) {
      dispatchSetPreQuestion('');
    }
  }
  
  return (
    <Tooltip
      placement='bottom'
      arrow={false}
      open={openExample}
      overlayStyle={{
        width: '760px',
        padding: '0',
        background: 'rgba(0,3,10,1.00)',
        borderRadius: '6px',
        maxWidth: 'unset',

      }}
      title={<Examples itemClick={()=> {
        setTimeout(()=> {
          setOpenExample(false);
        }, 10);
      }}/>}>
      <span className={styles.wrap}>
        <span className={styles.prefix}>
          <DatabendSvg></DatabendSvg>
        </span>
        <input 
          onClick={(e)=>{
            e.stopPropagation();
            setOpenExample(!isFeatching);
          }}
          id={INPUT_ID}
          value={queryText}
          autoComplete="off"
          onChange={(e)=> changeQueryText(e)}
          placeholder='Ask AI a question?' 
          className={clsx(styles.input, isSwitch && styles.inputOthview, isRegenerate && styles.inputRegenerate)} type='text' />
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
    </Tooltip>
  );
};
export default QuestionInput;
