import { useMemo } from "react";
import { useAppStore } from "../store/appStore";
import type { PreviewTabKey } from "../types/attendance";
import { DataTable } from "./DataTable";

export function PreviewPanel() {
  const preview = useAppStore((state) => state.preview);
  const activeTab = useAppStore((state) => state.activeTab);
  const runPreview = useAppStore((state) => state.runPreview);
  const runSummary = useAppStore((state) => state.runSummary);
  const runNotice = useAppStore((state) => state.runNotice);

  const tableModel = useMemo(() => buildTableModel(activeTab, preview), [activeTab, preview]);

  return (
    <section className="card span-3">
      <div className="card-header">
        <div>
          <h2>解析结果预览</h2>
          <p>先做解析预览，再生成汇总与通报名单。大表格默认分页渲染。</p>
        </div>
        <div className="button-row">
          <button className="secondary-button" onClick={() => void runPreview()} type="button">
            解析预览
          </button>
          <button className="primary-button" onClick={() => void runSummary()} type="button">
            生成汇总
          </button>
          <button className="secondary-button" onClick={() => void runNotice()} type="button">
            生成通报名单
          </button>
        </div>
      </div>

      <div className="preview-top-grid">
        <div className="mini-panel">
          <h3>工作表</h3>
          <ul className="compact-list">
            {preview.worksheetPreviews.map((item) => (
              <li key={`${item.fileName}-${item.sheetName}`}>
                <strong>{item.fileName}</strong>
                <span>{item.sheetName}</span>
                <span>{item.rowCount} 行 / {item.columnCount} 列</span>
              </li>
            ))}
          </ul>
        </div>
        <div className="mini-panel">
          <h3>人员块样本</h3>
          <ul className="compact-list">
            {preview.sampleBlocks.map((item) => (
              <li key={`${item.name}-${item.sourceFile}-${item.sheetName}`}>
                <strong>{item.name}</strong>
                <span>{item.sourceFile}/{item.sheetName}</span>
                <span>{item.dayCount} 天 / {item.tokenCount} token</span>
              </li>
            ))}
          </ul>
        </div>
      </div>

      {preview.warnings.length > 0 ? (
        <div className="warning-box">
          {preview.warnings.map((warning) => (
            <p key={warning}>{warning}</p>
          ))}
        </div>
      ) : null}

      <DataTable columns={tableModel.columns} rows={tableModel.rows} title={tableModel.title} />
    </section>
  );
}

function buildTableModel(activeTab: PreviewTabKey, preview: ReturnType<typeof useAppStore.getState>["preview"]) {
  switch (activeTab) {
    case "detail":
      return {
        title: "明细表",
        columns: [
          { key: "name", label: "姓名" },
          { key: "date", label: "日期" },
          { key: "amHit", label: "AM命中" },
          { key: "noonHit", label: "NOON命中" },
          { key: "dailyCount", label: "当日计次" },
          { key: "amTimes", label: "AM时间列表" },
          { key: "noonTimes", label: "NOON时间列表" },
        ],
        rows: preview.detailRows,
      };
    case "need-days":
      return {
        title: "需要打卡日",
        columns: [
          { key: "year", label: "年份" },
          { key: "month", label: "月份" },
          { key: "day", label: "需要打卡日" },
        ],
        rows: preview.needDayRows,
      };
    case "notice":
      return {
        title: "通报名单",
        columns: [
          { key: "name", label: "姓名" },
          { key: "needPunchDays", label: "需要打卡日" },
          { key: "expectedPunchCount", label: "应打卡次数" },
          { key: "actualPunchDays", label: "打卡天数" },
          { key: "actualPunchCount", label: "打卡次数" },
          { key: "absentDays", label: "缺勤天数" },
          { key: "absentCount", label: "缺勤次数" },
          { key: "absentDates", label: "缺勤具体日期" },
          { key: "triggerReason", label: "触发原因" },
        ],
        rows: preview.noticeRows,
      };
    case "summary":
    default:
      return {
        title: "汇总表",
        columns: [
          { key: "name", label: "姓名" },
          { key: "needPunchDays", label: "需要打卡日" },
          { key: "expectedPunchCount", label: "应打卡次数" },
          { key: "actualPunchDays", label: "打卡天数" },
          { key: "actualPunchCount", label: "打卡次数" },
          { key: "absentDays", label: "缺勤天数" },
          { key: "absentCount", label: "缺勤次数" },
          { key: "absentDates", label: "缺勤具体日期" },
        ],
        rows: preview.summaryRows,
      };
  }
}
