import { useAppStore } from "../store/appStore";

interface ProgressBannerProps {
  loading: boolean;
}

export function ProgressBanner({ loading }: ProgressBannerProps) {
  const progressMessage = useAppStore((state) => state.progressMessage);
  const progressPercent = useAppStore((state) => state.progressPercent);

  return (
    <section className={`progress-banner ${loading ? "is-running" : ""}`}>
      <div>
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
