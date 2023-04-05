import { ReactNode } from 'react';

export interface ICommonProps {
  children: ReactNode | string;
  className?: string;
  style?: React.CSSProperties;
  onClick?: ()=> void;
}


export type tResultsItem = {
  value: string;
  date?: number | string;
  isRegenerate?: boolean;
}