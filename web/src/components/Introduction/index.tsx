// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
const Introduction: FC = (): ReactElement=> {
  return (
    <div className={styles.introduction}>
      <p>
      🙋 I’m AskBend, I can help you with automatic search and completion for articles and knowledge base related to Databend. I have limitations and won’t always get it right, but your feedback will help me improve.
      </p>
    </div>
  );
};
export default Introduction;