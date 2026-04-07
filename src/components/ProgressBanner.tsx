import { useAppStore } from "../store/appStore";

export function ProgressBanner() {
  const loading = useAppStore((state) => state.loading);
  const progressMessage = useAppStore((state) => state.progressMessage);
  const progressPercent = useAppStore((state) => state.progressPercent);

  return (
    <section className={`progress-banner ${loading ? "is-running" : ""}`}>
      <div>
        <div className="panel-kicker">运行状态</div>
        <strong>{loading ? "后台任务执行中" : "等待执行"}</strong>
        <p>{progressMessage}</p>
      </div>
      <div className="progress-track">
        <div className="progress-fill" style={{ width: `${progressPercent}%` }} />
      </div>
      <span>{progressPercent}%</span>
    </section>
  );
}
