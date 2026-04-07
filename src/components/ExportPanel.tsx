import { useMemo, useState } from "react";
import { useAppStore } from "../store/appStore";

export function ExportPanel() {
  const { year, month } = useAppStore((state) => ({ year: state.year, month: state.month }));
  const exportSummaryXlsx = useAppStore((state) => state.exportSummaryXlsx);
  const exportSummaryCsv = useAppStore((state) => state.exportSummaryCsv);
  const exportNotice = useAppStore((state) => state.exportNotice);
  const [includeDetail, setIncludeDetail] = useState(true);
  const [includeNeedDays, setIncludeNeedDays] = useState(true);
  const [includeNotice, setIncludeNotice] = useState(true);
  const [summaryXlsxPath, setSummaryXlsxPath] = useState(`exports/汇总_${year}${String(month).padStart(2, "0")}.xlsx`);
  const [summaryCsvPath, setSummaryCsvPath] = useState(`exports/汇总_${year}${String(month).padStart(2, "0")}.csv`);
  const [noticePath, setNoticePath] = useState(`exports/通报名单_${year}${String(month).padStart(2, "0")}.xlsx`);

  const summaryHint = useMemo(
    () => `默认文件名已包含年月，可按需修改导出路径。`,
    [],
  );

  return (
    <section className="card span-2">
      <div className="card-header">
        <div>
          <h2>导出</h2>
          <p>{summaryHint}</p>
        </div>
      </div>

      <div className="check-grid">
        <label><input checked={includeDetail} type="checkbox" onChange={(event) => setIncludeDetail(event.target.checked)} />导出明细</label>
        <label><input checked={includeNeedDays} type="checkbox" onChange={(event) => setIncludeNeedDays(event.target.checked)} />导出需要打卡日</label>
        <label><input checked={includeNotice} type="checkbox" onChange={(event) => setIncludeNotice(event.target.checked)} />导出通报名单 sheet</label>
      </div>

      <div className="export-stack">
        <label>
          <span>汇总 xlsx 路径</span>
          <input value={summaryXlsxPath} onChange={(event) => setSummaryXlsxPath(event.target.value)} />
        </label>
        <div className="button-row">
          <button className="primary-button" onClick={() => void exportSummaryXlsx(summaryXlsxPath, includeDetail, includeNeedDays, includeNotice)} type="button">
            导出汇总 xlsx
          </button>
        </div>
        <label>
          <span>汇总 csv 路径</span>
          <input value={summaryCsvPath} onChange={(event) => setSummaryCsvPath(event.target.value)} />
        </label>
        <div className="button-row">
          <button className="secondary-button" onClick={() => void exportSummaryCsv(summaryCsvPath, includeDetail, includeNeedDays, includeNotice)} type="button">
            导出汇总 csv
          </button>
        </div>
        <label>
          <span>通报名单路径</span>
          <input value={noticePath} onChange={(event) => setNoticePath(event.target.value)} />
        </label>
        <div className="button-row">
          <button className="secondary-button" onClick={() => void exportNotice(noticePath)} type="button">
            导出通报名单
          </button>
        </div>
      </div>
    </section>
  );
}
