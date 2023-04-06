// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
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
        Powered by <a className={styles.link} target="_blank" href="https://www.databend.com" rel="noreferrer">Databend Cloud </a>
        and <a className={styles.link} target="_blank" href="https://databend.rs/doc/sql-functions/ai-functions/" rel="noreferrer">AI Functions</a>
        </span>
      }
    </div>
   
  );
};
ProductName.defaultProps = {
  showSponser: true
};
export default ProductName;