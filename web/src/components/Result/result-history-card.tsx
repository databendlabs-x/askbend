// Copyright 2023 Datafuse Labs.
import React, { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import TimeSvg from '../../assets/svg/time';
import ResultCard from './result-card';
import { timeFormatAgo } from '../../utils/tools';
interface IProps {
  value: string;
  date: string | number | undefined;
}
const ResultHistoryCard: FC<IProps> = ({value, date}): ReactElement=> {
  return (
    <div>
      <div className={styles.historyTitle}>
        <TimeSvg />
        <div>
          <span>Historical result</span>
          <span> {timeFormatAgo(date as number)}</span>
        </div>
      </div>
      <ResultCard value={value} />
    </div>
  );
};
export default ResultHistoryCard;