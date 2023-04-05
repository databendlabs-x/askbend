// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import Card from 'components/Card';
import clsx from 'clsx';
import AskDatabendMarkdown from 'components/Markdown';
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