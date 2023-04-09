import { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import copy from 'copy-to-clipboard';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
// cb coldarkDark okaidia tomorrow xonokai darcula
import { okaidia } from 'react-syntax-highlighter/dist/esm/styles/prism';
import remarkGfm from 'remark-gfm';
import RightSvg from '@/assets/svg/right';
import styles from './styles.module.scss';
import { deviceType } from '@/utils/device-type';

type tProps = {
  textContent: string
}
const AskDatabendMarkdown = (props: tProps) => {
  const { textContent } = props;
  const [isCopy, setIsCopy] = useState(false);
  const { isPhone } = deviceType();
  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        code({ inline, className, children, ...props }) {
          const match = /language-(\w+)/.exec(className || '');
          const text =  String(children).replace(/\n$/, '');
          return !inline && match ? (
            <div 
              onMouseLeave={()=> setIsCopy(false)}
              className={styles.codeWrap}>
              <SyntaxHighlighter
                showLineNumbers={true}
                style={okaidia as any}
                language={match[1]}
                PreTag='div'
                {...props}
              >
                {text}
              </SyntaxHighlighter>
              <span
                className={styles.copy}
                onClick={() => {
                  copy(text);
                  setIsCopy(true);
                }}
              >
                {
                  (isCopy && !isPhone)
                    ? <RightSvg />
                    : <>Copy</>
                }
              </span>
            </div>
          ) : (
            <code className={className} {...props}>
              {children}
            </code>
          );
        },
        a: (props: {href: string, children: string[]} | any) => {
          const desc = props?.children[0];
          return (
            <a 
              target="_blank" 
              title={desc} 
              rel="noopener noreferrer" 
              href={props?.href}>
              {props?.children[0]}
            </a>
          );
        }
      }}
    >
      {textContent}
    </ReactMarkdown>
  );
};

export default AskDatabendMarkdown;