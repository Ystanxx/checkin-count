import { useEffect, useState } from "react";
import { useAppStore } from "../store/appStore";
import type { AttendanceRules, NoticeRules } from "../types/attendance";
import { ProgressBanner } from "./ProgressBanner";
import { SimpleHome } from "./SimpleHome";
import { AdvancedMenu } from "./AdvancedMenu";
import { subscribeProgress } from "../utils/tauri";
import { saveAppPreferences } from "../utils/preferences";

export function AppShell() {
  const files = useAppStore((state) => state.inputFiles);
  const preview = useAppStore((state) => state.preview);
  const year = useAppStore((state) => state.year);
  const month = useAppStore((state) => state.month);
  const startRow = useAppStore((state) => state.startRow);
  const restDays = useAppStore((state) => state.restDays);
  const rules = useAppStore((state) => state.rules);
  const noticeRules = useAppStore((state) => state.noticeRules);
  const [advancedOpen, setAdvancedOpen] = useState(false);

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

  useEffect(() => {
    saveAppPreferences({
      year,
      month,
      startRow,
      restDays,
      rules: rules as AttendanceRules,
      noticeRules: noticeRules as NoticeRules,
    });
  }, [year, month, startRow, restDays, rules, noticeRules]);

  return (
    <div className="app-shell-simple">
      <header className="topbar">
        <div className="topbar-copy">
          <div>
            <div className="brand-kicker">本地离线 · Tauri v2 · Rust</div>
            <div className="brand-mark brand-mark-inline">团队打卡数据处理</div>
          </div>
        </div>
        <div className="topbar-stats">
          <div className="stat-card">
            <span>文件</span>
            <strong>{files.length}</strong>
          </div>
          <div className="stat-card">
            <span>人员</span>
            <strong>{preview.stats.people}</strong>
          </div>
          <div className="stat-card">
            <span>待通报</span>
            <strong>{preview.noticeRows.length}</strong>
          </div>
        </div>
      </header>

      <main className="main-panel">
        <ProgressBanner />
        <SimpleHome onOpenAdvanced={() => setAdvancedOpen(true)} />
      </main>
      <AdvancedMenu onClose={() => setAdvancedOpen(false)} open={advancedOpen} />
    </div>
  );
}
