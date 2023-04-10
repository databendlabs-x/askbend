// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
import SuccessTip from './success-tip';
import ResultCard from './result-card';
import ResultHistoryCard from './result-history-card';
import WaitCard from 'components/WaitCard';
import { useGetResultsState } from '@/state/hooks/useGetResultsState';
import Introduction from 'components/Introduction';
import useGetScrollInfo from '@/hooks/useGetScrollInfo';
import ErrorTip from '../Error';
import { tResultsItem } from '@/types';
const Result: FC = (): ReactElement=> {
  const { resultsList, isFeatching, showErrorTip } =  useGetResultsState();
  const { isSwitch } =  useGetScrollInfo();
  return (
    <>
      {
        showErrorTip && <ErrorTip />
      }
      <div className={styles.resultArea} style={{paddingTop: isSwitch ? `${document.getElementById('askbend_top_banner')?.offsetHeight}px`:'unset'}}>
        {
          isFeatching  && <WaitCard />
        }
        {
          resultsList?.length > 0 
            ? <>{!isFeatching && <SuccessTip />}</>
            : <>{isFeatching===undefined && <Introduction />}</>
        }
        {
          resultsList?.length>0 && 
          <div className={styles.list}>
            {
              resultsList?.map((res: tResultsItem, index: number)=> {
                if (index ==0) {
                  return  <ResultCard key={index} isFirst value={res.value} question={res?.question}/>;
                } else {
                  return  <ResultHistoryCard key={index} date={res.date} value={res.value}/>;
                }
              })
            }
          </div>
        }
      </div>
    </>
   
  );
};
export default Result;