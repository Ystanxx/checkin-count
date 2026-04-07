import { useAppStore } from "../store/appStore";
import type { PreviewTabKey } from "../types/attendance";

const tabs: Array<{ key: PreviewTabKey; label: string }> = [
  { key: "summary", label: "汇总" },
  { key: "detail", label: "明细" },
  { key: "need-days", label: "需要打卡日" },
  { key: "notice", label: "通报名单" },
];

interface TabsProps {
  activeTab: PreviewTabKey;
}

export function Tabs({ activeTab }: TabsProps) {
  const setActiveTab = useAppStore((state) => state.setActiveTab);

  return (
    <section className="card tabs-card">
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
    </section>
  );
}
