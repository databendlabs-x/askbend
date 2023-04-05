// Copyright 2023 Datafuse Labs.
import { FC, ReactElement, useEffect } from 'react';
import styles from './styles.module.scss';
import Card from 'components/Card';
import Typed from 'react-typed';
import { useSafeState, useInterval, useUnmount, useMount } from 'ahooks';
import { useGetResultsState } from '@/state/hooks/useGetResultsState';
const WaitCard: FC = (): ReactElement=> {
  const { isFeatching } = useGetResultsState();
  const [typeSpeed, setTypeSpeed] = useSafeState(100);
  const [interval, setInterval] = useSafeState<number | undefined>(undefined);
  useInterval(()=> {
    const randomNumber = Math.floor(Math.random() * 71) + 200;
    setTypeSpeed(randomNumber);
  }, interval);
  useMount(()=> {
    setInterval(3000);
  });
  useUnmount(()=> {
    setInterval(undefined);
  });
  useEffect(()=> {
    if (isFeatching === false) {
      setTypeSpeed(0);
      setInterval(undefined);
    }
  }, [isFeatching]);
  return (
    <Card className={styles.card} padding={[12, 12]}>
      <span>💡 </span>
      <Typed 
        className={styles.typed}
        loop={false}
        strings={['<span class="type-line-one">Please be patient and wait ... </span></br><span class="type-line-two">We are organizing the answers for you, which may take some time.</span>']}
        typeSpeed={typeSpeed}
      ></Typed>
    </Card>
  );
};
export default WaitCard;