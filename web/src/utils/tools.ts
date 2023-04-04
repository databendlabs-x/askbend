import dayjs from 'dayjs';
import { useHotkeys } from 'react-hotkeys-hook';
import relativeTime from 'dayjs/plugin/relativeTime';
import isYesterday from 'dayjs/plugin/isYesterday';
dayjs.extend(relativeTime);
dayjs.extend(isYesterday);

// Calculate the time between now and before
export function timeFormatAgo(time: number | string) {
  if (!time) return '-';
  const t = dayjs(time);
  return t.isYesterday() ? 'yesterday' : t.fromNow();
}

export function selfUseHotkeys(key:string, callback: ()=> void) {
  useHotkeys(key, (e: any)=> {
    e.preventDefault();
    callback();
  });
}