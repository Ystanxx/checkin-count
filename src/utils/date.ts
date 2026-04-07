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

export function buildWeekendDays(year: number, month: number) {
  return buildMonthDayList(year, month).filter((day) => {
    const current = new Date(year, month - 1, day).getDay();
    return current === 0 || current === 6;
  });
}
