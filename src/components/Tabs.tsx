import { useAppStore } from "../store/appStore";
import type { PreviewTabKey } from "../types/attendance";

const tabs: Array<{ key: PreviewTabKey; label: string }> = [
  { key: "summary", label: "汇总" },
  { key: "detail", label: "明细" },
  { key: "need-days", label: "需要打卡日" },
  { key: "notice", label: "通报名单" },
];

export function Tabs() {
  const activeTab = useAppStore((state) => state.activeTab);
  const setActiveTab = useAppStore((state) => state.setActiveTab);

  return (
    <div className="tabs-row">
      {tabs.map((tab) => (
        <button
          className={`tab-button ${activeTab === tab.key ? "is-active" : ""}`}
          key={tab.key}
          onClick={() => setActiveTab(tab.key)}
          type="button"
        >
          {tab.label}
        </button>
      ))}
    </div>
  );
}
