import ReactMarkdown from 'react-markdown';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
// cb coldarkDark okaidia tomorrow xonokai darcula
import { okaidia } from 'react-syntax-highlighter/dist/esm/styles/prism';
import remarkGfm from 'remark-gfm';

type tProps = {
  textContent: string
}
const AskDatabendMarkdown = (props: tProps) => {
  const { textContent } = props;
  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        code({ node, inline, className, children, ...props }) {
          const match = /language-(\w+)/.exec(className || '');
          return !inline && match ? (
            <SyntaxHighlighter
              showLineNumbers={true}
              style={okaidia as any}
              language={match[1]}
              PreTag='div'
              {...props}
            >
              {String(children).replace(/\n$/, '')}
            </SyntaxHighlighter>
          ) : (
            <code className={className} {...props}>
              {children}
            </code>
          );
        }
      }}
    >
      {textContent}
    </ReactMarkdown>
  );
};

export default AskDatabendMarkdown;