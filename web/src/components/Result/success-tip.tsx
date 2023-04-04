// Copyright 2023 Datafuse Labs.
import React,{ FC, ReactElement } from 'react';
import styles from './styles.module.less';
import CheckSvg from '../../assets/svg/check';
const SuccessTip: FC = (): ReactElement=> {
  return (
    <div>
      <CheckSvg></CheckSvg>
      <span className={styles.successTips}>Here are the results we found for you.</span>
    </div>
  );
};
export default SuccessTip;