// Copyright 2023 Datafuse Labs.
import { FC, ReactElement, useEffect } from 'react';
import styles from './styles.module.scss';
import Card from 'components/Card';
import Typed from 'react-typed';
import { useSafeState, useInterval, useUnmount, useMount } from 'ahooks';
import { useGetResultsState } from '@/state/hooks/useGetResultsState';
const WaitCard: FC = (): ReactElement=> {
  const { isFeatching, preQuestion, inputQuestion } = useGetResultsState();
  const [typeSpeed, setTypeSpeed] = useSafeState(50);
  const [interval, setInterval] = useSafeState<number | undefined>(undefined);
  const [question, setQuestion] = useSafeState('');
  useInterval(()=> {
    const randomNumber = Math.floor(Math.random() * 50) + 20;
    setTypeSpeed(randomNumber);
  }, interval);
  useMount(()=> {
    setInterval(2000);
  });
  useUnmount(()=> {
    setInterval(undefined);
  });
  useEffect(()=> {
    if (isFeatching === false) {
      setInterval(undefined);
    }
  }, [isFeatching]);
  useEffect(()=> {
    preQuestion && setQuestion(preQuestion);
  }, [preQuestion]);
  useEffect(()=> {
    inputQuestion && setQuestion(inputQuestion);
  }, [inputQuestion]);
  return (
    <Card className={styles.card} padding={[12, 12]}>
      <span>💡 </span>
      <Typed 
        className={styles.typed}
        loop={false}
        strings={[`
          <span class="type-line-question">You seem curious about: ${question}</span></br><span class="type-line-two">AI is in the process of organizing the answers for you, which may take some time.</span>
        `]}
        typeSpeed={typeSpeed}
      ></Typed>
    </Card>
  );
};
export default WaitCard;