// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.less';
const ErrorTip: FC = (): ReactElement=> {
  return (
    <div className={styles.wrap}>
      I&apos;m sorry, the generation failed. Please generate again.
    </div>
  );
};
export default ErrorTip;