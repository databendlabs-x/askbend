// Copyright 2023 Datafuse Labs.
import React, { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import Card from '../Card';
const WaitCard: FC = (): ReactElement=> {
  return (
    <Card className={styles.card} padding={[12, 12]}>
      <div>💡 Please be patient and wait ... </div>
      <div>
        We are organizing the answers for you, which may take some time.
      </div>
    </Card>
  );
};
export default WaitCard;