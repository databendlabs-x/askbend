// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
import clsx from 'clsx';
interface IProps {
  queryText: string;
}
const CountLength: FC<IProps> = ({queryText}): ReactElement=> {
  return (
    <span className={styles.countLimit}>
      <span className={clsx(queryText?.length >= 500 && styles.maxLength)}>{queryText?.length}</span> / 500
    </span>
  );
};
export default CountLength;