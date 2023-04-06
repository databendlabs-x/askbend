// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
import PreviewJson from '@/assets/json/preview-question.json';
import Card from 'components/Card';
import useResultsDispatch from '@/state/redux/dispatch/result';
const Introduction: FC = (): ReactElement=> {
  const { dispatchSetPreQuestion, dispatchSetInputQuestion } = useResultsDispatch();
  function selectedPreQuestion(content: string) {
    dispatchSetInputQuestion('');
    dispatchSetPreQuestion(content);
  }
  return (
    <div className={styles.introduction}>
      <p>
      🙋 I’m AskBend, I can help you with automatic search and completion for articles and knowledge base related to Databend. I have limitations and won’t always get it right, but your feedback will help me improve.
      </p>
      <div className={styles.introPhone}>
        <p className={styles.introTitle}>Not sure where to start? You can try:</p>
        <div className={styles.list}>
          {
            PreviewJson?.map((prev, index) => {
              return (
                <Card 
                  onClick={()=> selectedPreQuestion(prev)}
                  padding={[10, 12]}
                  key={index}>
                  <span>🤔️ </span>
                  <span>{prev}</span>
                </Card>
              );
            })
          }
        </div>
      </div>
    </div>
  );
};
export default Introduction;