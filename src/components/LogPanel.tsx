import { useAppStore } from "../store/appStore";

export function LogPanel() {
  const logs = useAppStore((state) => state.logs);

  return (
    <section className="card log-card workspace-logs">
      <div className="card-header">
        <div>
          <div className="panel-kicker">审计轨迹</div>
          <h2>运行日志</h2>
          <p>默认日志不展示完整绝对路径与完整个人数据。</p>
        </div>
      </div>
      <div className="log-list">
        {logs.map((log) => (
          <article className={`log-item level-${log.level}`} key={log.id}>
            <span>{new Date(log.timestamp).toLocaleTimeString()}</span>
            <strong>{log.level.toUpperCase()}</strong>
            <p>{log.message}</p>
          </article>
        ))}
      </div>
    </section>
  );
}
