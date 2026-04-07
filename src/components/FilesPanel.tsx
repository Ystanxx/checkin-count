import { useAppStore } from "../store/appStore";
import { ellipsis } from "../utils/format";

export function FilesPanel() {
  const files = useAppStore((state) => state.inputFiles);
  const loadInputFiles = useAppStore((state) => state.loadInputFiles);

  return (
    <section className="card workspace-files">
      <div className="card-header">
        <div>
          <div className="panel-kicker">输入源</div>
          <h2>文件选择</h2>
          <p>文件路径通过 Tauri 命令选择，前端只持有展示所需信息。</p>
        </div>
        <button className="primary-button" onClick={() => void loadInputFiles()} type="button">
          选择输入文件
        </button>
      </div>
      <div className="file-list">
        {files.length === 0 ? (
          <div className="empty-state">尚未选择文件。</div>
        ) : (
          files.map((item) => (
            <article className="file-item" key={item.path}>
              <div>
                <strong>{item.displayName}</strong>
                <div className="muted">{ellipsis(item.path, 72)}</div>
              </div>
              <span className="file-tag">本地文件</span>
            </article>
          ))
        )}
      </div>
    </section>
  );
}
