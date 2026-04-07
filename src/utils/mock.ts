import type {
  AttendanceRules,
  DetailRow,
  InputFileItem,
  NeedDayRow,
  NoticeRow,
  NoticeRules,
  PreviewData,
  SummaryRow,
} from "../types/attendance";
import { buildMonthDayList } from "./date";

export function createMockInputFiles(): InputFileItem[] {
  return [
    {
      path: "本地示例/一月打卡.xlsx",
      displayName: "一月打卡.xlsx",
    },
    {
      path: "本地示例/二月打卡.xlsm",
      displayName: "二月打卡.xlsm",
    },
  ];
}

export function createMockRules(): AttendanceRules {
  return {
    amStart: "00:00:00",
    amEnd: "09:11:59",
    noonStart: "11:00:00",
    noonEnd: "14:11:59",
  };
}

export function createMockNoticeRules(): NoticeRules {
  return {
    absentDaysThreshold: 3,
    absentCountThreshold: 5,
    operator: "OR",
  };
}

function createMockSummaryRow(index: number): SummaryRow {
  const needPunchDays = 22;
  const actualPunchDays = Math.max(0, needPunchDays - index);
  const actualPunchCount = actualPunchDays * 2 - Math.min(index, 2);
  const absentDays = needPunchDays - actualPunchDays;
  const absentCount = needPunchDays * 2 - actualPunchCount;
  return {
    name: `员工${index + 1}`,
    needPunchDays,
    expectedPunchCount: needPunchDays * 2,
    actualPunchDays,
    actualPunchCount,
    absentDays,
    absentCount,
    absentDates: absentDays > 0 ? "3,5,8" : "",
  };
}

function createMockDetailRows(summaryRows: SummaryRow[]): DetailRow[] {
  const rows: DetailRow[] = [];
  summaryRows.forEach((summary) => {
    for (let day = 1; day <= 12; day += 1) {
      rows.push({
        name: summary.name,
        date: `2026-04-${String(day).padStart(2, "0")}`,
        day,
        amHit: day % 2 === 0,
        noonHit: day % 3 === 0,
        dailyCount: Number(day % 2 === 0) + Number(day % 3 === 0),
        amTimes: day % 2 === 0 ? "08:55:12" : "",
        noonTimes: day % 3 === 0 ? "12:08:00" : "",
      });
    }
  });
  return rows;
}

function createMockNeedDays(): NeedDayRow[] {
  return buildMonthDayList(2026, 4).map((day) => ({
    year: 2026,
    month: 4,
    day,
  }));
}

function createMockNoticeRows(summaryRows: SummaryRow[]): NoticeRow[] {
  return summaryRows.slice(0, 5).map((row, index) => ({
    ...row,
    triggerReason: index % 2 === 0 ? `缺勤天数>${index}` : `缺勤次数>${index + 1}`,
  }));
}

export function createMockPreviewData(): PreviewData {
  const summaryRows = Array.from({ length: 12 }, (_, index) => createMockSummaryRow(index));
  return {
    recognizedNames: summaryRows.map((row) => row.name),
    worksheetPreviews: [
      {
        fileName: "一月打卡.xlsx",
        sheetName: "打卡",
        rowCount: 62,
        columnCount: 34,
      },
      {
        fileName: "二月打卡.xlsm",
        sheetName: "总表",
        rowCount: 31,
        columnCount: 28,
      },
    ],
    sampleBlocks: summaryRows.slice(0, 6).map((row, index) => ({
      name: row.name,
      dayCount: 22,
      tokenCount: 35 - index,
      sourceFile: index % 2 === 0 ? "一月打卡.xlsx" : "二月打卡.xlsm",
      sheetName: index % 2 === 0 ? "打卡" : "总表",
    })),
    summaryRows,
    detailRows: createMockDetailRows(summaryRows),
    needDayRows: createMockNeedDays(),
    noticeRows: createMockNoticeRows(summaryRows),
    warnings: ["当前使用前端兜底示例数据，尚未连接 Rust 命令。"],
    stats: {
      files: 2,
      sheets: 2,
      people: summaryRows.length,
      blocks: 12,
      records: summaryRows.length * 12,
    },
  };
}
