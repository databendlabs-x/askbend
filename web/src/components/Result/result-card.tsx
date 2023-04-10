// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
import Card from 'components/Card';
import clsx from 'clsx';
import copy from 'copy-to-clipboard';
import AskDatabendMarkdown from 'components/Markdown';
import LinkIcon from '@/assets/svg/link';
import { notification } from 'antd';
interface IProps{
  isFirst?: boolean;
  value: string;
  question?: string;
}
const ResultCard: FC<IProps> = ({isFirst, value, question}): ReactElement=> {
  function shareWithOthers() {
    copy(`https://ask.databend.rs/?q=${encodeURIComponent(question as string)}`);
    notification.open({
      message: 'Tips',
      duration: 1.5,
      placement: 'topLeft',
      description:
        'The share link is already in the clipboard'
    });
  }
  return (
    <Card
      className={clsx(styles.cardWrap, isFirst && styles.cardWrapOn)}
      padding={[20, 20]}>
      {
        isFirst &&
        <div 
          onClick={()=> shareWithOthers()}
          className={styles.share} title='Share with others'>
          <LinkIcon></LinkIcon>
        </div>
      }
      <AskDatabendMarkdown textContent={value} />
    </Card>
  );
};
export default ResultCard;