import type { LogEntry } from "../types/attendance";

interface LogPanelProps {
  logs: LogEntry[];
}

export function LogPanel({ logs }: LogPanelProps) {
  return (
    <section className="card log-card">
      <div className="card-header">
        <div>
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
