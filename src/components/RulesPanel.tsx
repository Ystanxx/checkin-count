import { useAppStore } from "../store/appStore";
import { buildMonthDayList } from "../utils/date";

function toggleRestDay(current: number[], day: number) {
  return current.includes(day)
    ? current.filter((item) => item !== day)
    : [...current, day].sort((left, right) => left - right);
}

export function RulesPanel() {
  const year = useAppStore((state) => state.year);
  const month = useAppStore((state) => state.month);
  const startRow = useAppStore((state) => state.startRow);
  const restDays = useAppStore((state) => state.restDays);
  const rules = useAppStore((state) => state.rules);
  const noticeRules = useAppStore((state) => state.noticeRules);
  const setYearMonth = useAppStore((state) => state.setYearMonth);
  const setStartRow = useAppStore((state) => state.setStartRow);
  const setRestDays = useAppStore((state) => state.setRestDays);
  const setRules = useAppStore((state) => state.setRules);
  const setNoticeRules = useAppStore((state) => state.setNoticeRules);

  const days = buildMonthDayList(year, month);

  return (
    <section className="card">
      <div className="card-header">
        <div>
          <h2>规则配置</h2>
          <p>默认口径采用旧代码真实行为，窗口可配置。</p>
        </div>
      </div>

      <div className="form-grid">
        <label>
          <span>年份</span>
          <input max={2100} min={2000} type="number" value={year} onChange={(event) => setYearMonth(Number(event.target.value), month)} />
        </label>
        <label>
          <span>月份</span>
          <input max={12} min={1} type="number" value={month} onChange={(event) => setYearMonth(year, Number(event.target.value))} />
        </label>
        <label>
          <span>数据起始行</span>
          <input min={1} type="number" value={startRow} onChange={(event) => setStartRow(Number(event.target.value))} />
        </label>
        <label>
          <span>AM 起始</span>
          <input value={rules.amStart} onChange={(event) => setRules({ ...rules, amStart: event.target.value })} />
        </label>
        <label>
          <span>AM 结束</span>
          <input value={rules.amEnd} onChange={(event) => setRules({ ...rules, amEnd: event.target.value })} />
        </label>
        <label>
          <span>NOON 起始</span>
          <input value={rules.noonStart} onChange={(event) => setRules({ ...rules, noonStart: event.target.value })} />
        </label>
        <label>
          <span>NOON 结束</span>
          <input value={rules.noonEnd} onChange={(event) => setRules({ ...rules, noonEnd: event.target.value })} />
        </label>
      </div>

      <div className="notice-rule-grid">
        <label>
          <span>缺勤天数阈值</span>
          <input
            min={0}
            type="number"
            value={noticeRules.absentDaysThreshold ?? ""}
            onChange={(event) =>
              setNoticeRules({
                ...noticeRules,
                absentDaysThreshold: event.target.value === "" ? null : Number(event.target.value),
              })
            }
          />
        </label>
        <label>
          <span>缺勤次数阈值</span>
          <input
            min={0}
            type="number"
            value={noticeRules.absentCountThreshold ?? ""}
            onChange={(event) =>
              setNoticeRules({
                ...noticeRules,
                absentCountThreshold: event.target.value === "" ? null : Number(event.target.value),
              })
            }
          />
        </label>
        <label>
          <span>组合逻辑</span>
          <select
            value={noticeRules.operator}
            onChange={(event) =>
              setNoticeRules({
                ...noticeRules,
                operator: event.target.value as "AND" | "OR",
              })
            }
          >
            <option value="OR">OR</option>
            <option value="AND">AND</option>
          </select>
        </label>
      </div>

      <div className="rest-days-grid">
        {days.map((day) => (
          <button
            className={`day-pill ${restDays.includes(day) ? "is-active" : ""}`}
            key={day}
            onClick={() => setRestDays(toggleRestDay(restDays, day))}
            type="button"
          >
            {day}
          </button>
        ))}
      </div>
    </section>
  );
}
