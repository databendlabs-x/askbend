// accountInfo state
import { THEME } from '@/assets/CONSTANT_VAR';
import { useLocalStorageState, useMount } from 'ahooks';
import { createContext } from 'react';
 
// create context
export const AccountInfoContext = createContext<any>({});
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import { toggleTheme } from '@zougt/vite-plugin-theme-preprocessor/dist/browser-utils';
import useSetOrGetUserInfo from '@/hooks/useSetOrGetUserInfo';
import useLayoutDispatch from './redux/dispatch/layout';
 
export const AccountInfo = (props:any) => {
  const { setUserInfo } = useSetOrGetUserInfo();
  const { dispatchSetTheme } = useLayoutDispatch();
  const [cacheTheme, setCacheTheme] = useLocalStorageState<string>(THEME.CACHE_THEME);
  useMount(()=>{
    try{
      if (cacheTheme) {
        toggleTheme({
          scopeName: cacheTheme,
        });
        dispatchSetTheme(cacheTheme);
      } else {
        setCacheTheme(THEME.DEFAULT);
        dispatchSetTheme(THEME.DEFAULT);
      }
    } catch{
      console.log('theme error');
    }
  });
  useMount(()=> {
    setUserInfo();
  });
  return (
    <AccountInfoContext.Provider value={{}}>
      {props.children}
    </AccountInfoContext.Provider>
  );
};