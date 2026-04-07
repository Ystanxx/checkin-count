import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type {
  DetailRow,
  InputFileItem,
  NeedDayRow,
  NoticeRow,
  PreviewData,
  SummaryRow,
} from "../types/attendance";
import type {
  CommandResult,
  RustBuildSummaryResponse,
  RustNoticeBuildResponse,
  RustPreviewResponse,
  RustProgressEvent,
} from "../types/commands";

export function isTauriAvailable() {
  return (
    typeof window !== "undefined" &&
    ("__TAURI__" in window || "__TAURI_INTERNALS__" in window)
  );
}

export async function callTauriCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<CommandResult<T>> {
  if (!isTauriAvailable()) {
    return {
      ok: false,
      message: `Tauri 环境不可用，命令 ${command} 仅在桌面壳内执行。`,
    };
  }

  try {
    const data = await invoke<T>(command, args ?? {});
    return { ok: true, data };
  } catch (error) {
    const message = error instanceof Error ? error.message : "Tauri 命令执行失败。";
    return { ok: false, message };
  }
}

export async function subscribeProgress(
  onEvent: (payload: RustProgressEvent) => void,
): Promise<() => void> {
  if (!isTauriAvailable()) {
    return () => undefined;
  }

  const unlisten = await listen<RustProgressEvent>("task://progress", (event) => {
    onEvent(event.payload);
  });
  return () => {
    void unlisten();
  };
}

export function toInputFiles(paths: string[]): InputFileItem[] {
  return paths.map((path) => ({
    path,
    displayName: path.split(/[\\/]/).pop() ?? path,
  }));
}

export function mapPreviewResponse(response: RustPreviewResponse): PreviewData {
  return {
    recognizedNames: response.recognized_names,
    worksheetPreviews: response.worksheet_previews.map((item) => ({
      fileName: item.file_name,
      sheetName: item.sheet_name,
      rowCount: item.row_count,
      columnCount: item.column_count,
    })),
    sampleBlocks: response.sample_blocks.map((item) => ({
      name: item.name,
      dayCount: item.day_count,
      tokenCount: item.token_count,
      sourceFile: item.source_file,
      sheetName: item.sheet_name,
    })),
    summaryRows: [],
    detailRows: [],
    needDayRows: [],
    noticeRows: [],
    warnings: response.warnings,
    stats: {
      files: uniqueCount(response.worksheet_previews.map((item) => item.file_name)),
      sheets: response.stats.worksheet_count,
      people: response.stats.recognized_name_count,
      blocks: response.stats.block_count,
      records: response.stats.valid_record_count,
    },
  };
}

export function applySummaryResponse(
  current: PreviewData,
  response: RustBuildSummaryResponse,
): PreviewData {
  return {
    ...current,
    summaryRows: response.summary_rows.map(mapSummaryRow),
    detailRows: response.detail_rows.map(mapDetailRow),
    needDayRows: response.need_day_rows.map(mapNeedDayRow),
    noticeRows: response.notice_rows.map(mapNoticeRow),
    warnings: response.warnings,
    stats: {
      files: current.stats.files,
      sheets: response.stats.worksheet_count,
      people: response.stats.recognized_name_count,
      blocks: response.stats.block_count,
      records: response.stats.valid_record_count,
    },
  };
}

export function applyNoticeResponse(
  current: PreviewData,
  response: RustNoticeBuildResponse,
): PreviewData {
  return {
    ...current,
    noticeRows: response.notice_rows.map(mapNoticeRow),
  };
}

function mapSummaryRow(row: RustBuildSummaryResponse["summary_rows"][number]): SummaryRow {
  return {
    name: row.name,
    needPunchDays: row.need_punch_days,
    expectedPunchCount: row.expected_punch_count,
    actualPunchDays: row.actual_punch_days,
    actualPunchCount: row.actual_punch_count,
    absentDays: row.absent_days,
    absentCount: row.absent_count,
    absentDates: row.absent_dates.join(","),
  };
}

function mapDetailRow(row: RustBuildSummaryResponse["detail_rows"][number]): DetailRow {
  return {
    name: row.name,
    date: row.date,
    day: row.day,
    amHit: row.am_hit,
    noonHit: row.noon_hit,
    dailyCount: row.daily_count,
    amTimes: row.am_times.join(","),
    noonTimes: row.noon_times.join(","),
  };
}

function mapNeedDayRow(row: RustBuildSummaryResponse["need_day_rows"][number]): NeedDayRow {
  return {
    year: row.year,
    month: row.month,
    day: row.day,
  };
}

function mapNoticeRow(row: RustBuildSummaryResponse["notice_rows"][number]): NoticeRow {
  return {
    name: row.name,
    needPunchDays: row.need_punch_days,
    expectedPunchCount: row.expected_punch_count,
    actualPunchDays: row.actual_punch_days,
    actualPunchCount: row.actual_punch_count,
    absentDays: row.absent_days,
    absentCount: row.absent_count,
    absentDates: row.absent_dates.join(","),
    triggerReason: row.trigger_reason,
  };
}

function uniqueCount(values: string[]) {
  return new Set(values).size;
}
