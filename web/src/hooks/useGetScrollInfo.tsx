import { useScroll } from 'ahooks';

const useGetScrollInfo = ( yourStand = 218)=> {
  const scrollInfo = useScroll();
  const top = scrollInfo?.top as number;
  const isSwitch = top > yourStand;
  return {
    isSwitch,
    top,
    scrollInfo
  };
};
export default useGetScrollInfo;