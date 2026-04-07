export interface InputFileItem {
  path: string;
  displayName: string;
}

export interface AttendanceRules {
  amStart: string;
  amEnd: string;
  noonStart: string;
  noonEnd: string;
}

export interface NoticeRules {
  absentDaysThreshold: number | null;
  absentCountThreshold: number | null;
  operator: "AND" | "OR";
}

export interface PreviewStats {
  files: number;
  sheets: number;
  people: number;
  blocks: number;
  records: number;
}

export interface WorksheetPreviewItem {
  fileName: string;
  sheetName: string;
  rowCount: number;
  columnCount: number;
}

export interface PreviewBlockItem {
  name: string;
  dayCount: number;
  tokenCount: number;
  sourceFile: string;
  sheetName: string;
}

export interface SummaryRow {
  name: string;
  needPunchDays: number;
  expectedPunchCount: number;
  actualPunchDays: number;
  actualPunchCount: number;
  absentDays: number;
  absentCount: number;
  absentDates: string;
}

export interface DetailRow {
  name: string;
  date: string;
  day: number;
  amHit: boolean;
  noonHit: boolean;
  dailyCount: number;
  amTimes: string;
  noonTimes: string;
}

export interface NeedDayRow {
  year: number;
  month: number;
  day: number;
}

export interface NoticeRow extends SummaryRow {
  triggerReason: string;
}

export interface PreviewData {
  recognizedNames: string[];
  worksheetPreviews: WorksheetPreviewItem[];
  sampleBlocks: PreviewBlockItem[];
  summaryRows: SummaryRow[];
  detailRows: DetailRow[];
  needDayRows: NeedDayRow[];
  noticeRows: NoticeRow[];
  warnings: string[];
  stats: PreviewStats;
}

export interface LogEntry {
  id: string;
  level: "info" | "warning" | "error" | "success";
  message: string;
  timestamp: string;
}

export type PreviewTabKey = "summary" | "detail" | "need-days" | "notice";

export interface AppState {
  inputFiles: InputFileItem[];
  year: number;
  month: number;
  startRow: number;
  restDays: number[];
  rules: AttendanceRules;
  noticeRules: NoticeRules;
  loading: boolean;
  progressMessage: string;
  progressPercent: number;
  activeTab: PreviewTabKey;
  preview: PreviewData;
  logs: LogEntry[];
}
