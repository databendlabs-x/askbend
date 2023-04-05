// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
const TimeSvg: FC = (): ReactElement=> {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M8 15C11.866 15 15 11.866 15 8C15 4.134 11.866 1 8 1C4.134 1 1 4.134 1 8C1 11.866 4.134 15 8 15Z" stroke="#0C162B" strokeOpacity="0.8" strokeLinejoin="round"/>
      <path d="M8.00286 3.80005L8.00244 8.00313L10.9702 10.9709" stroke="#0C162B" strokeOpacity="0.8" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
    
  );
};
export default TimeSvg;