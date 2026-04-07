import { useMemo, useState } from "react";

export function usePreviewPagination(totalCount: number, pageSize = 12) {
  const [page, setPage] = useState(1);

  const pageCount = Math.max(1, Math.ceil(totalCount / pageSize));
  const safePage = Math.min(page, pageCount);
  const start = (safePage - 1) * pageSize;
  const end = Math.min(start + pageSize, totalCount);

  const range = useMemo(
    () => ({ start, end, pageCount, page: safePage }),
    [start, end, pageCount, safePage],
  );

  return {
    ...range,
    pageSize,
    setPage,
    nextPage: () => setPage((current) => Math.min(current + 1, pageCount)),
    prevPage: () => setPage((current) => Math.max(current - 1, 1)),
  };
}
