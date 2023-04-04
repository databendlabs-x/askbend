// Copyright 2023 Datafuse Labs.
import React, { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import Card from '../Card';
import clsx from 'clsx';
import AskDatabendMarkdown from '../Markdown';
interface IProps{
  isFirst?: boolean;
  value: string;
}
const ResultCard: FC<IProps> = ({isFirst, value}): ReactElement=> {
  return (
    <Card
      className={clsx(styles.cardWrap, isFirst && styles.cardWrapOn)}
      padding={[20, 20]}>
      <AskDatabendMarkdown textContent={value} />
    </Card>
  );
};
export default ResultCard;