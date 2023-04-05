import { deviceType } from '@/utils/device-type';
import { useScroll } from 'ahooks';

const useGetScrollInfo = ( yourStand = 218)=> {
  const scrollInfo = useScroll();
  const top = scrollInfo?.top as number;
  let isSwitch = false;
  const { isPhone } = deviceType();
  if (isPhone) {
    isSwitch = false;
  } else {
    isSwitch = top > yourStand;
  }
  return {
    isSwitch,
    top,
    scrollInfo
  };
};
export default useGetScrollInfo;