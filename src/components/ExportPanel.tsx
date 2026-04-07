import { useMemo, useState } from "react";
import { useAppStore } from "../store/appStore";

export function ExportPanel() {
  const year = useAppStore((state) => state.year);
  const month = useAppStore((state) => state.month);
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
    <section className="card workspace-export">
      <div className="card-header">
        <div>
          <div className="panel-kicker">输出台</div>
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
        <label className="export-block">
          <span>汇总 xlsx 路径</span>
          <input value={summaryXlsxPath} onChange={(event) => setSummaryXlsxPath(event.target.value)} />
          <small>适合正式汇总与留档。</small>
        </label>
        <div className="button-row">
          <button className="primary-button" onClick={() => void exportSummaryXlsx(summaryXlsxPath, includeDetail, includeNeedDays, includeNotice)} type="button">
            导出汇总 xlsx
          </button>
        </div>
        <label className="export-block">
          <span>汇总 csv 路径</span>
          <input value={summaryCsvPath} onChange={(event) => setSummaryCsvPath(event.target.value)} />
          <small>带 UTF-8 BOM，便于 Excel 直接打开。</small>
        </label>
        <div className="button-row">
          <button className="secondary-button" onClick={() => void exportSummaryCsv(summaryCsvPath, includeDetail, includeNeedDays, includeNotice)} type="button">
            导出汇总 csv
          </button>
        </div>
        <label className="export-block">
          <span>通报名单路径</span>
          <input value={noticePath} onChange={(event) => setNoticePath(event.target.value)} />
          <small>适合独立发给管理端。</small>
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
