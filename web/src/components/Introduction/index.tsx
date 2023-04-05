// Copyright 2023 Datafuse Labs.
import React,{ FC, ReactElement } from 'react';
import styles from './styles.module.less';
import PreviewJson from '@/assets/json/preview-question.json';
import Card from 'components/Card';
import useResultsDispatch from '@/state/redux/dispatch/result';
const Introduction: FC = (): ReactElement=> {
  const { dispatchSetPreQuestion } = useResultsDispatch();
  function selectedPreQuestion(content: string) {
    dispatchSetPreQuestion(content);
  }
  return (
    <div className={styles.introduction}>
      <p>
      🙋 I’m AskBend, I can help you with automatic search and completion for articles and knowledge base related to Databend. I have limitations and won’t always get it right, but your feedback will help me improve.
      </p>
      <p className={styles.introTitle}>Not sure where to start? You can try:</p>
      <div className={styles.list}>
        {
          PreviewJson?.map((prev, index) => {
            return (
              <Card 
                onClick={()=> selectedPreQuestion(prev)}
                padding={[10, 12]}
                key={index}>
                🤔️ {prev}
              </Card>
            );
          })
        }
      </div>
    </div>
  );
};
export default Introduction;