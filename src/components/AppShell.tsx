import { useEffect, useMemo, useState } from "react";
import { useAppStore } from "../store/appStore";
import { FileSelectPage } from "../pages/FileSelectPage";
import { RulesPage } from "../pages/RulesPage";
import { PreviewPage } from "../pages/PreviewPage";
import { ExportPage } from "../pages/ExportPage";
import { LogPanel } from "./LogPanel";
import { ProgressBanner } from "./ProgressBanner";
import { Tabs } from "./Tabs";
import { subscribeProgress } from "../utils/tauri";

export function AppShell() {
  const loading = useAppStore((state) => state.loading);
  const logs = useAppStore((state) => state.logs);
  const preview = useAppStore((state) => state.preview);
  const activeTab = useAppStore((state) => state.activeTab);
  const setProgress = useAppStore((state) => state.setProgress);
  const appendLog = useAppStore((state) => state.appendLog);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  useEffect(() => {
    let cleanup: (() => void) | undefined;
    void subscribeProgress((payload) => {
      setProgress(payload.message, payload.percent);
      appendLog("info", `${payload.stage}: ${payload.message}`);
    }).then((unlisten) => {
      cleanup = unlisten;
    });
    return () => {
      cleanup?.();
    };
  }, [appendLog, setProgress]);

  const quickStats = useMemo(
    () => [
      { label: "文件", value: preview.stats.files },
      { label: "工作表", value: preview.stats.sheets },
      { label: "人员", value: preview.stats.people },
      { label: "记录", value: preview.stats.records },
    ],
    [preview.stats],
  );

  return (
    <div className="app-shell">
      <aside className={`sidebar ${sidebarCollapsed ? "is-collapsed" : ""}`}>
        <div className="brand-block">
          <div>
            <div className="brand-kicker">Tauri v2 + Rust + React</div>
            <div className="brand-mark">团队打卡数据处理</div>
          </div>
          <button
            className="ghost-button"
            onClick={() => setSidebarCollapsed((value) => !value)}
            type="button"
          >
            {sidebarCollapsed ? "展开" : "收起"}
          </button>
        </div>
        <div className="sidebar-stats">
          {quickStats.map((item) => (
            <div className="stat-card" key={item.label}>
              <span>{item.label}</span>
              <strong>{item.value}</strong>
            </div>
          ))}
        </div>
        <div className="sidebar-note">
          本地离线处理，不接入远程 API，不在前端解析 Excel。
        </div>
      </aside>

      <main className="main-panel">
        <ProgressBanner loading={loading} />
        <section className="page-grid">
          <FileSelectPage />
          <RulesPage />
          <PreviewPage />
          <ExportPage />
        </section>
        <section className="bottom-grid">
          <Tabs activeTab={activeTab} />
          <LogPanel logs={logs} />
        </section>
      </main>
    </div>
  );
}
