// Copyright 2023 Datafuse Labs.
import React from 'react';
import { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import ProductName from '../ProcuctName';
import QuestionInput from '../Input';
const Banner: FC = (): ReactElement=> {
  return (
    <div className={styles.banner}>
      <div className={styles.content}>
        <ProductName />
        <QuestionInput />
      </div>
    </div>
  );
};
export default Banner;