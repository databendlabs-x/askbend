// Copyright 2023 Datafuse Labs.
import React from 'react';
import { FC, ReactElement } from 'react';
import styles from './styles.module.less';
const ProductName: FC = (): ReactElement=> {
  return (
    <div className={styles.wrap}>
      <div className={styles.product}>
        <span>Ask</span><span className={styles.bend}>Bend</span>
      </div>
      <span className={styles.sponser}>
        Power by <a className={styles.link} target="_blank" href="https://www.databend.com" rel="noreferrer">Databend Cloud</a>
      </span>
    </div>
   
  );
};
export default ProductName;