import { useEffect, useMemo, useState } from "react";
import { useAppStore } from "../store/appStore";
import { buildMonthDayList, buildWeekendDays } from "../utils/date";
import { buildNoticePath, loadAppPreferences, saveAppPreferences } from "../utils/preferences";

interface SimpleHomeProps {
  onOpenAdvanced: () => void;
}

function mergeDays(current: number[], next: number[]) {
  return Array.from(new Set([...current, ...next])).sort((left, right) => left - right);
}

export function SimpleHome({ onOpenAdvanced }: SimpleHomeProps) {
  const files = useAppStore((state) => state.inputFiles);
  const year = useAppStore((state) => state.year);
  const month = useAppStore((state) => state.month);
  const restDays = useAppStore((state) => state.restDays);
  const loading = useAppStore((state) => state.loading);
  const preview = useAppStore((state) => state.preview);
  const loadInputFiles = useAppStore((state) => state.loadInputFiles);
  const runPrimaryFlow = useAppStore((state) => state.runPrimaryFlow);
  const exportNotice = useAppStore((state) => state.exportNotice);
  const setYearMonth = useAppStore((state) => state.setYearMonth);
  const setRestDays = useAppStore((state) => state.setRestDays);

  const [noticePath, setNoticePath] = useState(
    loadAppPreferences()?.noticePath ?? buildNoticePath(year, month),
  );

  const monthDays = useMemo(() => buildMonthDayList(year, month), [year, month]);
  const weekendDays = useMemo(() => buildWeekendDays(year, month), [year, month]);
  const noticeRows = preview.noticeRows;
  const topNotices = noticeRows.slice(0, 8);

  useEffect(() => {
    const defaultPath = buildNoticePath(year, month);
    setNoticePath((currentPath) => {
      const savedPath = loadAppPreferences()?.noticePath;
      const currentDefault = buildNoticePath(year, month);
      if (!currentPath || currentPath === savedPath || currentPath === currentDefault) {
        return defaultPath;
      }
      return currentPath;
    });
  }, [year, month]);

  useEffect(() => {
    saveAppPreferences({ noticePath });
  }, [noticePath]);

  async function handleProcess() {
    await runPrimaryFlow();
  }

  async function handleExportNotice() {
    await exportNotice(noticePath);
  }

  return (
    <div className="simple-home">
      <section className="hero-card">
        <div>
          <div className="eyebrow">本地离线处理</div>
          <h1>导入 Excel，选择休息日，导出通报名单。</h1>
          <p>
            默认只保留最常用流程。规则配置、预览、日志和更多导出在高级菜单里。
          </p>
        </div>
        <button className="secondary-button" onClick={onOpenAdvanced} type="button">
          高级菜单
        </button>
      </section>

      <section className="simple-grid">
        <section className="flow-card">
          <div className="step-badge">步骤 1</div>
          <h2>导入考勤文件</h2>
          <p>支持 `xls`、`xlsx`、`xlsm`，可一次选择多个文件。</p>
          <button className="primary-button" onClick={() => void loadInputFiles()} type="button">
            选择 Excel 文件
          </button>
          <div className="selected-files">
            {files.length === 0 ? (
              <div className="empty-hint">还没有选择文件。</div>
            ) : (
              files.map((item) => (
                <div className="file-pill" key={item.path}>
                  {item.displayName}
                </div>
              ))
            )}
          </div>
        </section>

        <section className="flow-card">
          <div className="step-badge">步骤 2</div>
          <h2>选择月份与休息日</h2>
          <p>先选年月，再用“自动勾选周末”补齐周末，节假日直接点日期即可。</p>
          <div className="quick-form">
            <label>
              <span>年份</span>
              <input
                max={2100}
                min={2000}
                onChange={(event) => setYearMonth(Number(event.target.value), month)}
                type="number"
                value={year}
              />
            </label>
            <label>
              <span>月份</span>
              <input
                max={12}
                min={1}
                onChange={(event) => setYearMonth(year, Number(event.target.value))}
                type="number"
                value={month}
              />
            </label>
          </div>
          <div className="quick-actions">
            <button
              className="secondary-button"
              onClick={() => setRestDays(mergeDays(restDays, weekendDays))}
              type="button"
            >
              自动勾选周末
            </button>
            <button className="ghost-button" onClick={() => setRestDays([])} type="button">
              清空休息日
            </button>
          </div>
          <div className="holiday-grid">
            {monthDays.map((day) => {
              const isWeekend = weekendDays.includes(day);
              const isActive = restDays.includes(day);
              return (
                <button
                  className={`day-chip ${isActive ? "is-active" : ""} ${isWeekend ? "is-weekend" : ""}`}
                  key={day}
                  onClick={() =>
                    setRestDays(
                      isActive
                        ? restDays.filter((item) => item !== day)
                        : mergeDays(restDays, [day]),
                    )
                  }
                  type="button"
                >
                  <span>{day}</span>
                  <small>{isWeekend ? "周末" : "工作日"}</small>
                </button>
              );
            })}
          </div>
        </section>

        <section className="flow-card process-card">
          <div className="step-badge">步骤 3</div>
          <h2>处理并导出</h2>
          <p>点击一次即可自动执行汇总分析并生成通报名单。</p>
          <div className="result-grid">
            <div className="result-chip">
              <span>已导入文件</span>
              <strong>{files.length}</strong>
            </div>
            <div className="result-chip">
              <span>已识别人员</span>
              <strong>{preview.stats.people}</strong>
            </div>
            <div className="result-chip">
              <span>需通报人数</span>
              <strong>{noticeRows.length}</strong>
            </div>
          </div>
          <div className="quick-actions">
            <button
              className="primary-button"
              disabled={loading || files.length === 0}
              onClick={() => void handleProcess()}
              type="button"
            >
              开始处理
            </button>
          </div>
          <label className="export-input-block">
            <span>通报名单导出路径</span>
            <input
              onChange={(event) => setNoticePath(event.target.value)}
              value={noticePath}
            />
          </label>
          <div className="quick-actions">
            <button
              className="secondary-button"
              disabled={loading || noticeRows.length === 0}
              onClick={() => void handleExportNotice()}
              type="button"
            >
              导出通报名单
            </button>
          </div>
          <div className="result-list">
            <div className="section-title">本次待通报</div>
            {topNotices.length === 0 ? (
              <div className="empty-hint">处理完成后，这里会显示待通报名单。</div>
            ) : (
              topNotices.map((item) => (
                <div className="notice-row" key={`${item.name}-${item.triggerReason}`}>
                  <strong>{item.name}</strong>
                  <span>{item.triggerReason}</span>
                </div>
              ))
            )}
          </div>
        </section>
      </section>
    </div>
  );
}
