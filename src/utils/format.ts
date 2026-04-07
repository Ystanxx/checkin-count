export function formatNumber(value: number) {
  return new Intl.NumberFormat("zh-CN").format(value);
}

export function formatDateText(year: number, month: number, day: number) {
  return `${year}-${String(month).padStart(2, "0")}-${String(day).padStart(
    2,
    "0",
  )}`;
}

export function ellipsis(value: string, maxLength = 24) {
  if (value.length <= maxLength) {
    return value;
  }
  return `${value.slice(0, maxLength - 1)}…`;
}
