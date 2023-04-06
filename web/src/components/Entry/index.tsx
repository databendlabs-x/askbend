// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import Banner from 'components/Banner/banner';
import Result from 'components/Result';
import Footer from '@/components/Footer';
const Entry: FC = (): ReactElement=> {
  return (
    <>
      <Banner />
      <Result />
      <Footer />
    </>
  );
};
export default Entry;