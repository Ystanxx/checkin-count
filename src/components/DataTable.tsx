import { useMemo } from "react";
import { usePreviewPagination } from "../store/usePreviewPagination";

interface ColumnDefinition {
  key: string;
  label: string;
}

interface DataTableProps {
  title: string;
  columns: ColumnDefinition[];
  rows: Array<Record<string, unknown>>;
}

export function DataTable({ title, columns, rows }: DataTableProps) {
  const pagination = usePreviewPagination(rows.length, 10);
  const currentRows = useMemo(
    () => rows.slice(pagination.start, pagination.end),
    [pagination.end, pagination.start, rows],
  );

  return (
    <div className="table-card">
      <div className="table-toolbar">
        <h3>{title}</h3>
        <span>
          第 {pagination.page}/{pagination.pageCount} 页，当前 {pagination.start + 1}-{pagination.end}
        </span>
      </div>
      <div className="table-wrap">
        <table>
          <thead>
            <tr>
              {columns.map((column) => (
                <th key={column.key}>{column.label}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {currentRows.length === 0 ? (
              <tr>
                <td className="empty-cell" colSpan={columns.length}>
                  当前无可展示数据。
                </td>
              </tr>
            ) : (
              currentRows.map((row, index) => (
                <tr key={`${title}-${pagination.start + index}`}>
                  {columns.map((column) => (
                    <td key={column.key}>{formatValue(row[column.key])}</td>
                  ))}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
      <div className="pager-row">
        <button onClick={pagination.prevPage} type="button">上一页</button>
        <button onClick={pagination.nextPage} type="button">下一页</button>
      </div>
    </div>
  );
}

function formatValue(value: unknown) {
  if (typeof value === "boolean") {
    return value ? "是" : "否";
  }
  if (value === null || value === undefined || value === "") {
    return "-";
  }
  return String(value);
}
