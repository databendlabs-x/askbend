// Copyright 2023 Datafuse Labs.
import React from 'react';
import { FC, ReactElement } from 'react';
import Banner from '../Banner/banner';
import styles from './styles.module.less';
import Result from '../Result';
import Footer from '../footer';
const Entry: FC = (): ReactElement=> {
  return (
    <>
      <Banner />
      <div className={styles.resultArea}>
        <Result />
      </div>
      <Footer />
    </>
  );
};
export default Entry;