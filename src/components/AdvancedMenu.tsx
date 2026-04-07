import { ExportPanel } from "./ExportPanel";
import { LogPanel } from "./LogPanel";
import { PreviewPanel } from "./PreviewPanel";
import { RulesPanel } from "./RulesPanel";

interface AdvancedMenuProps {
  open: boolean;
  onClose: () => void;
}

export function AdvancedMenu({ open, onClose }: AdvancedMenuProps) {
  if (!open) {
    return null;
  }

  return (
    <div className="advanced-overlay" onClick={onClose} role="presentation">
      <aside
        aria-label="高级菜单"
        className="advanced-drawer"
        onClick={(event) => event.stopPropagation()}
      >
        <div className="advanced-header">
          <div>
            <div className="eyebrow">高级菜单</div>
            <h2>更多配置与调试信息</h2>
            <p>这里保留规则配置、预览、日志和更多导出选项。配置会自动保存在本地。</p>
          </div>
          <button className="ghost-button" onClick={onClose} type="button">
            关闭
          </button>
        </div>

        <details className="advanced-section" open>
          <summary>规则配置</summary>
          <RulesPanel />
        </details>

        <details className="advanced-section">
          <summary>更多导出</summary>
          <ExportPanel />
        </details>

        <details className="advanced-section">
          <summary>结果预览</summary>
          <PreviewPanel />
        </details>

        <details className="advanced-section">
          <summary>运行日志</summary>
          <LogPanel />
        </details>
      </aside>
    </div>
  );
}
