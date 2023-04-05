// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.less';
import ProductName from 'components/ProcuctName';
import QuestionInput from 'components/Input';
import clsx from 'clsx';
import useGetScrollInfo from '@/hooks/useGetScrollInfo';
const Banner: FC = (): ReactElement=> {
  const { isSwitch } =  useGetScrollInfo();
  return (
    <>
      {
        isSwitch
          ? <div className={clsx(styles.banner, styles.secondBanner)}>
            <div className={styles.content}>
              <ProductName showSponser={false} />
              <QuestionInput />
            </div>
          </div>
          :  <div className={styles.banner}>
            <div className={styles.content}>
              <ProductName />
              <QuestionInput />
            </div>
          </div>
       
      }
    </>
  );
};
export default Banner;