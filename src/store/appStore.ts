import { create } from "zustand";
import type {
  AppState,
  AttendanceRules,
  InputFileItem,
  LogEntry,
  NoticeRules,
  PreviewData,
  PreviewTabKey,
} from "../types/attendance";
import { getCurrentYearMonth } from "../utils/date";
import { createMockInputFiles, createMockNoticeRules, createMockPreviewData, createMockRules } from "../utils/mock";
import {
  applyNoticeResponse,
  applySummaryResponse,
  callTauriCommand,
  mapPreviewResponse,
  toInputFiles,
} from "../utils/tauri";
import type {
  RustBuildSummaryResponse,
  RustNoticeBuildResponse,
  RustPreviewResponse,
} from "../types/commands";

function createLogEntry(level: LogEntry["level"], message: string): LogEntry {
  return {
    id: crypto.randomUUID(),
    level,
    message,
    timestamp: new Date().toISOString(),
  };
}

interface AppActions {
  setInputFiles: (files: InputFileItem[]) => void;
  setRules: (rules: AttendanceRules) => void;
  setNoticeRules: (rules: NoticeRules) => void;
  setYearMonth: (year: number, month: number) => void;
  setStartRow: (value: number) => void;
  setRestDays: (days: number[]) => void;
  setPreview: (preview: PreviewData) => void;
  setActiveTab: (tab: PreviewTabKey) => void;
  setLoading: (loading: boolean) => void;
  setProgress: (message: string, percent: number) => void;
  appendLog: (level: LogEntry["level"], message: string) => void;
  loadInputFiles: () => Promise<void>;
  runPreview: () => Promise<void>;
  runSummary: () => Promise<void>;
  runNotice: () => Promise<void>;
  exportSummaryXlsx: (outputPath: string, includeDetail: boolean, includeNeedDays: boolean, includeNotice: boolean) => Promise<void>;
  exportSummaryCsv: (outputPath: string, includeDetail: boolean, includeNeedDays: boolean, includeNotice: boolean) => Promise<void>;
  exportNotice: (outputPath: string) => Promise<void>;
  resetToMock: () => void;
}

const current = getCurrentYearMonth();
const initialPreview = createMockPreviewData();

export const useAppStore = create<AppState & AppActions>((set, get) => ({
  inputFiles: [],
  year: current.year,
  month: current.month,
  startRow: 1,
  restDays: [],
  rules: createMockRules(),
  noticeRules: createMockNoticeRules(),
  loading: false,
  progressMessage: "等待执行",
  progressPercent: 0,
  activeTab: "summary",
  preview: initialPreview,
  logs: [createLogEntry("info", "前端已就绪，等待桌面命令接入。")],
  setInputFiles: (files) => set({ inputFiles: files }),
  setRules: (rules) => set({ rules }),
  setNoticeRules: (rules) => set({ noticeRules: rules }),
  setYearMonth: (year, month) => set({ year, month }),
  setStartRow: (value) => set({ startRow: value }),
  setRestDays: (days) => set({ restDays: days }),
  setPreview: (preview) => set({ preview }),
  setActiveTab: (tab) => set({ activeTab: tab }),
  setLoading: (loading) => set({ loading }),
  setProgress: (message, percent) => set({ progressMessage: message, progressPercent: percent }),
  appendLog: (level, message) =>
    set((state) => ({ logs: [...state.logs, createLogEntry(level, message)] })),
  loadInputFiles: async () => {
    const { appendLog } = get();
    set({ loading: true, progressMessage: "正在打开文件对话框", progressPercent: 10 });
    const result = await callTauriCommand<string[]>("select_input_files");
    if (result.ok && result.data) {
      set({ inputFiles: toInputFiles(result.data), loading: false, progressMessage: "文件选择完成", progressPercent: 100 });
      appendLog("success", `已选择 ${result.data.length} 个输入文件。`);
      return;
    }

    set({ inputFiles: createMockInputFiles(), loading: false, progressMessage: "切换到示例文件", progressPercent: 100 });
    appendLog("warning", result.message ?? "未连接到桌面命令，已切换到示例文件。");
  },
  runPreview: async () => {
    const state = get();
    set({ loading: true, progressMessage: "正在生成解析预览", progressPercent: 15 });
    const result = await callTauriCommand<RustPreviewResponse>("parse_attendance_preview", {
      request: {
        input_files: state.inputFiles.map((item) => item.path),
        start_row: state.startRow,
      },
    });
    if (result.ok && result.data) {
      set({ preview: mapPreviewResponse(result.data), loading: false, progressMessage: "解析预览完成", progressPercent: 100 });
      state.appendLog("success", "解析预览已更新。")
      return;
    }

    set({ preview: createMockPreviewData(), loading: false, progressMessage: "切换到示例预览", progressPercent: 100 });
    state.appendLog("warning", result.message ?? "解析预览失败，已切换到示例预览。");
  },
  runSummary: async () => {
    const state = get();
    set({ loading: true, progressMessage: "正在构建汇总结果", progressPercent: 20 });
    const result = await callTauriCommand<RustBuildSummaryResponse>("build_summary", {
      request: {
        input_files: state.inputFiles.map((item) => item.path),
        year: state.year,
        month: state.month,
        start_row: state.startRow,
        rules: {
          am_start: state.rules.amStart,
          am_end: state.rules.amEnd,
          noon_start: state.rules.noonStart,
          noon_end: state.rules.noonEnd,
          rest_days: state.restDays,
        },
      },
    });
    if (result.ok && result.data) {
      set({
        preview: applySummaryResponse(state.preview, result.data),
        activeTab: "summary",
        loading: false,
        progressMessage: "汇总构建完成",
        progressPercent: 100,
      });
      state.appendLog("success", "汇总结果已刷新。");
      return;
    }

    set({ preview: createMockPreviewData(), loading: false, progressMessage: "切换到示例汇总", progressPercent: 100 });
    state.appendLog("warning", result.message ?? "汇总构建失败，已切换到示例数据。");
  },
  runNotice: async () => {
    const state = get();
    set({ loading: true, progressMessage: "正在生成通报名单", progressPercent: 30 });
    const result = await callTauriCommand<RustNoticeBuildResponse>("build_notice_list", {
      request: {
        rules: {
          absent_days_threshold: state.noticeRules.absentDaysThreshold,
          absent_count_threshold: state.noticeRules.absentCountThreshold,
          operator: state.noticeRules.operator,
        },
      },
    });
    if (result.ok && result.data) {
      set({
        preview: applyNoticeResponse(state.preview, result.data),
        activeTab: "notice",
        loading: false,
        progressMessage: "通报名单生成完成",
        progressPercent: 100,
      });
      state.appendLog("success", "通报名单已刷新。");
      return;
    }

    set({ loading: false, progressMessage: "通报名单生成失败", progressPercent: 100 });
    state.appendLog("warning", result.message ?? "通报名单生成失败。请先生成汇总结果。\n");
  },
  exportSummaryXlsx: async (outputPath, includeDetail, includeNeedDays, includeNotice) => {
    const state = get();
    set({ loading: true, progressMessage: "正在导出 xlsx", progressPercent: 60 });
    const result = await callTauriCommand<string>("export_summary_xlsx", {
      request: {
        output_path: outputPath,
        include_detail: includeDetail,
        include_need_days: includeNeedDays,
        include_notice: includeNotice,
      },
    });
    set({ loading: false, progressMessage: "xlsx 导出结束", progressPercent: 100 });
    state.appendLog(result.ok ? "success" : "error", result.ok ? `已导出 xlsx：${result.data}` : result.message ?? "xlsx 导出失败。");
  },
  exportSummaryCsv: async (outputPath, includeDetail, includeNeedDays, includeNotice) => {
    const state = get();
    set({ loading: true, progressMessage: "正在导出 csv", progressPercent: 60 });
    const result = await callTauriCommand<string>("export_summary_csv", {
      request: {
        output_path: outputPath,
        include_detail: includeDetail,
        include_need_days: includeNeedDays,
        include_notice: includeNotice,
      },
    });
    set({ loading: false, progressMessage: "csv 导出结束", progressPercent: 100 });
    state.appendLog(result.ok ? "success" : "error", result.ok ? `已导出 csv：${result.data}` : result.message ?? "csv 导出失败。");
  },
  exportNotice: async (outputPath) => {
    const state = get();
    set({ loading: true, progressMessage: "正在导出通报名单", progressPercent: 70 });
    const result = await callTauriCommand<string>("export_notice_list", {
      request: {
        output_path: outputPath,
      },
    });
    set({ loading: false, progressMessage: "通报名单导出结束", progressPercent: 100 });
    state.appendLog(result.ok ? "success" : "error", result.ok ? `已导出通报名单：${result.data}` : result.message ?? "通报名单导出失败。");
  },
  resetToMock: () =>
    set({
      preview: createMockPreviewData(),
      inputFiles: [],
      rules: createMockRules(),
      noticeRules: createMockNoticeRules(),
      progressMessage: "已切换到示例数据",
      progressPercent: 100,
    }),
}));
