// Copyright 2023 Datafuse Labs.
import { FC, ReactElement } from 'react';
import styles from './styles.module.scss';
import GithubSvg from '@/assets/svg/github';
import { numberFormat } from '@/utils/tools';
import repoJson from '@/assets/json/repo-info.json';
const Footer: FC = (): ReactElement=> {
  return (
    <div className={styles.footer}>
      <span>SQL-based Knowledge Base Search and Completion using Databend</span>
      <span>|</span>
      <a href='https://github.com/datafuselabs/askbend' target='_blank' rel="noreferrer">
        <span className={styles.start}><GithubSvg /> <span className={styles.number}>Star: {numberFormat(repoJson?.stargazers_count??100)}</span></span>
      </a>
    </div>
  );
};
export default Footer;