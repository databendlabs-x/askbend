// Copyright 2023 Datafuse Labs.
import React, { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import SuccessTip from './success-tip';
import ResultCard from './result-card';
import ResultHistoryCard from './result-history-card';
import WaitCard from '../WaitCard';
import { useGetResultsState } from '../../state/hooks/useGetResultsState';
import Introduction from '../Introduction';
const Result: FC = (): ReactElement=> {
  const { resultsList, isFeatching } =  useGetResultsState();
  return (
    <div>
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
    </div>
  );
};
export default Result;