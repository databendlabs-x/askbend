// Copyright 2023 Datafuse Labs.
import React from 'react';
import { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import useGetScrollInfo from '@/hooks/useGetScrollInfo';
import clsx from 'clsx';
interface IProps {
  showSponser?: boolean;
}
const ProductName: FC<IProps> = ({showSponser}): ReactElement=> {
  const { isSwitch } =  useGetScrollInfo();
  return (
    <div className={clsx(styles.wrap, isSwitch && styles.wrapOtherView)}>
      <div className={styles.product}>
        <span>Ask</span><span className={styles.bend}>Bend</span>
      </div>
      {
        showSponser && 
        <span className={styles.sponser}>
        Powered by <a className={styles.link} target="_blank" href="https://www.databend.com" rel="noreferrer">Databend Cloud</a>
        </span>
      }
    </div>
   
  );
};
ProductName.defaultProps = {
  showSponser: true
};
export default ProductName;