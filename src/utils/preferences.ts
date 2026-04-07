import type { AttendanceRules, NoticeRules } from "../types/attendance";

const STORAGE_KEY = "attendance.local.preferences.v1";

export interface AppPreferences {
  year: number;
  month: number;
  startRow: number;
  restDays: number[];
  rules: AttendanceRules;
  noticeRules: NoticeRules;
  noticePath: string;
}

export function loadAppPreferences(): Partial<AppPreferences> | null {
  if (typeof window === "undefined") {
    return null;
  }

  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return null;
    }
    return JSON.parse(raw) as Partial<AppPreferences>;
  } catch {
    return null;
  }
}

export function saveAppPreferences(preferences: Partial<AppPreferences>) {
  if (typeof window === "undefined") {
    return;
  }

  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(preferences));
  } catch {
    // 本地配置保存失败时静默处理，避免影响主流程。
  }
}

export function buildNoticePath(year: number, month: number) {
  return `exports/通报名单_${year}${String(month).padStart(2, "0")}.xlsx`;
}
