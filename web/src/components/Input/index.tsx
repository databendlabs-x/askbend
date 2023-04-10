// Copyright 2023 Datafuse Labs.
import { useEffect, useRef, useState } from 'react';
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
import { getQuery, scrollToTop } from '@/utils/tools';
import { getAnswers } from '@/api';
import { Tooltip } from 'antd';
import Examples from './examples';
import { deviceType } from '@/utils/device-type';
import CountLength from './count-length';
const QuestionInput: FC = (): ReactElement=> {
  const TEXTAREA_ID = 'question-input';
  const q = getQuery('q') || '';
  const { isPhone } = deviceType();
  const { isSwitch } =  useGetScrollInfo();
  const { dispatchUpdateResultList, dispatchIsFetching, dispatchShowErrorTip, dispatchSetInputQuestion, dispatchSetPreQuestion } =  useResultsDispatch();
  const [queryText, setQueryText] = useSafeState(q);
  const { isFeatching, preQuestion } = useGetResultsState();
  const [isRegenerate, setIsRegenerate] = useState(false);
  const [openExample, setOpenExample] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement | null>(null);
  useMount(()=> {
    if (q) {
      dispatchSetInputQuestion(q);
      getResults(false, q);
    }
  });
  useEffect(()=>{
    window.addEventListener('keydown', onKeyDown);
    return ()=>{
      window.removeEventListener('keydown', onKeyDown);
    };
  }, [queryText, isFeatching, isRegenerate]);
  useEffect(() => {
    if (textareaRef.current) {
      autoResize();
    }
  }, []);
  useEffect(()=> {
    autoResize();
  }, [isSwitch]);
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
    if ((e.target as HTMLInputElement)?.id === TEXTAREA_ID && code === KEY_CODE.ENTER) {
      setOpenExample(false);
      e.preventDefault();
      if (isFeatching) return;
      getResults(isRegenerate);
    }
  }
  async function getResults(isRegenerate = false, preQuestion?: string) {
    dispatchIsFetching(true);
    dispatchShowErrorTip(false);
    setOpenExample(false);
    try{
      const q = preQuestion ? preQuestion : queryText;
      const data = await getAnswers(preQuestion ? preQuestion : queryText);
      if ([200, 201]?.includes(data?.status )) {
        const res = data?.data?.result;
        dispatchUpdateResultList({value: res, isRegenerate, question: q});
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
  function changeQueryText(e: React.ChangeEvent<HTMLTextAreaElement>) {
    const value = e?.target?.value;
    setQueryText(value);
    setIsRegenerate(false);
    dispatchSetInputQuestion(value);
    setOpenExample(true);
    dispatchSetPreQuestion('');
  }
  function autoResize() {
    const textarea = textareaRef?.current;
    if (textarea) {
      textarea.style.height = 'auto';
      textarea.style.height = textarea.scrollHeight + 'px';
    }
  }
  
  return (
    <Tooltip
      placement='bottom'
      arrow={false}
      open={openExample}
      overlayStyle={{
        width: isPhone ? '96%' : (isSwitch ? '557px' : '760px'),
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
          <DatabendSvg />
        </span>
        <textarea 
          ref={textareaRef}
          onClick={(e)=>{
            e.stopPropagation();
            setOpenExample(!isFeatching);
          }}
          maxLength={500}
          id={TEXTAREA_ID}
          rows={2}
          value={queryText}
          autoComplete="off"
          onInput={autoResize}
          onChange={(e)=> changeQueryText(e)}
          placeholder='Ask AI a question?' 
          className={clsx(
            styles.textarea, 
            isSwitch && styles.textareaOthview, 
            isRegenerate && styles.textareaRegenerate,
            isFeatching && styles.textareaFetching
          )} />
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
        <CountLength queryText={queryText}/>
      </span>
    </Tooltip>
  );
};
export default QuestionInput;
