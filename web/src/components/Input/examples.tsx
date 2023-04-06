// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
import PreviewJson from '@/assets/json/preview-question.json';
import useResultsDispatch from '@/state/redux/dispatch/result';
interface IProps {
  itemClick: ()=> void;
}
const Examples: FC<IProps> = ({itemClick}): ReactElement=> {
  const { dispatchSetPreQuestion, dispatchSetInputQuestion } = useResultsDispatch();
  function selectedPreQuestion(content: string) {
    dispatchSetInputQuestion('');
    dispatchSetPreQuestion(content);
    itemClick();
  }
  return (
    <div className={styles.examples} onClick={(e)=> {
      e.stopPropagation();
    }}>
      <div className={styles.header}>Examples</div>
      <div className={styles.list}>
        {
          PreviewJson?.map((question, index)=> {
            return (
              <div onClick={()=> selectedPreQuestion(question)} className={styles.item} key={index}>{question}</div>
            );
          })
        }
      </div>
    </div>
  );
};
export default Examples;