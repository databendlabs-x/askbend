// Copyright 2023 Datafuse Labs.
import React, { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import SuccessTip from './success-tip';
import ResultCard from './result-card';
import ResultHistoryCard from './result-history-card';
import WaitCard from 'components/WaitCard';
import { useGetResultsState } from '@/state/hooks/useGetResultsState';
import Introduction from 'components/Introduction';
import useGetScrollInfo from '@/hooks/useGetScrollInfo';
import ErrorTip from '../Error';
const Result: FC = (): ReactElement=> {
  const { resultsList, isFeatching, showErrorTip } =  useGetResultsState();
  const { isSwitch } =  useGetScrollInfo();
  return (
    <div className={styles.resultArea} style={{paddingTop: isSwitch ? '218px':'unset'}}>
      {
        isFeatching  && <WaitCard />
      }
      {
        resultsList?.length > 0 
          ? <>{!isFeatching && <SuccessTip />}</>
          : <>{isFeatching===undefined && <Introduction />}</>
      }
      <div className={styles.list}>
        {
          resultsList?.map((res,index)=> {
            if (index ==0) {
              return  <ResultCard key={index} isFirst value={res.value}/>;
            } else {
              return  <ResultHistoryCard key={index} date={res.date} value={res.value}/>;
            }
          })
        }
      </div>
      {
        showErrorTip && <ErrorTip />
      }
    </div>
  );
};
export default Result;