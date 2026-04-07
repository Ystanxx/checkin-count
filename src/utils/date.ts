export function getCurrentYearMonth() {
  const now = new Date();
  return {
    year: now.getFullYear(),
    month: now.getMonth() + 1,
  };
}

export function buildMonthDayList(year: number, month: number) {
  const daysInMonth = new Date(year, month, 0).getDate();
  return Array.from({ length: daysInMonth }, (_, index) => index + 1);
}
