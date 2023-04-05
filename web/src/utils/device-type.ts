interface DeviceType {
  type: 'mobile' | 'pc';
  isPhone: boolean;
}
function isMoible(UA: string) {
  return !!/(Android|webOS|iPhone|iPod|tablet|BlackBerry|Mobile)/i.test(UA);
}

export function deviceType(): DeviceType {
  let type: DeviceType['type'] = 'pc';
  const detaultDeviceType: DeviceType = { type, isPhone: false };
  const UA = navigator.userAgent??'';
  if (isMoible(UA)) {
    type = 'mobile';
    return {
      type, isPhone: true
    };
  } else {
    return detaultDeviceType;
  }
}
