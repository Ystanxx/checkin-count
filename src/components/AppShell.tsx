import { useEffect, useMemo, useState } from "react";
import { useAppStore } from "../store/appStore";
import { FileSelectPage } from "../pages/FileSelectPage";
import { RulesPage } from "../pages/RulesPage";
import { PreviewPage } from "../pages/PreviewPage";
import { ExportPage } from "../pages/ExportPage";
import { LogPanel } from "./LogPanel";
import { ProgressBanner } from "./ProgressBanner";
import { subscribeProgress } from "../utils/tauri";

export function AppShell() {
  const preview = useAppStore((state) => state.preview);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  useEffect(() => {
    let disposed = false;
    let cleanup: (() => void) | undefined;

    void subscribeProgress((payload) => {
      const store = useAppStore.getState();
      const message = `${payload.stage}: ${payload.message}`;
      const latestLog = store.logs[store.logs.length - 1];

      store.setProgress(payload.message, payload.percent);
      if (!latestLog || latestLog.message !== message) {
        store.appendLog("info", message);
      }
    }).then((unlisten) => {
      if (disposed) {
        unlisten();
        return;
      }
      cleanup = unlisten;
    });

    return () => {
      disposed = true;
      cleanup?.();
    };
  }, []);

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
    <div className={`app-shell ${sidebarCollapsed ? "is-sidebar-collapsed" : ""}`}>
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
        <ProgressBanner />
        <section className="workspace-grid">
          <FileSelectPage />
          <ExportPage />
          <RulesPage />
          <PreviewPage />
          <LogPanel />
        </section>
      </main>
    </div>
  );
}
