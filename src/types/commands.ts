export interface RustPreviewResponse {
  recognized_names: string[];
  worksheet_previews: Array<{
    file_name: string;
    sheet_name: string;
    row_count: number;
    column_count: number;
  }>;
  sample_blocks: Array<{
    name: string;
    day_count: number;
    token_count: number;
    source_file: string;
    sheet_name: string;
  }>;
  warnings: string[];
  stats: {
    worksheet_count: number;
    recognized_name_count: number;
    block_count: number;
    raw_token_count: number;
    valid_record_count: number;
  };
}

export interface RustBuildSummaryResponse {
  summary_rows: Array<{
    name: string;
    need_punch_days: number;
    expected_punch_count: number;
    actual_punch_days: number;
    actual_punch_count: number;
    absent_days: number;
    absent_count: number;
    absent_dates: number[];
  }>;
  detail_rows: Array<{
    name: string;
    date: string;
    day: number;
    am_hit: boolean;
    noon_hit: boolean;
    daily_count: number;
    am_times: string[];
    noon_times: string[];
  }>;
  need_day_rows: Array<{
    year: number;
    month: number;
    day: number;
  }>;
  notice_rows: Array<{
    name: string;
    need_punch_days: number;
    expected_punch_count: number;
    actual_punch_days: number;
    actual_punch_count: number;
    absent_days: number;
    absent_count: number;
    absent_dates: number[];
    trigger_reason: string;
  }>;
  warnings: string[];
  stats: {
    worksheet_count: number;
    recognized_name_count: number;
    block_count: number;
    raw_token_count: number;
    valid_record_count: number;
  };
}

export interface RustNoticeBuildResponse {
  notice_rows: RustBuildSummaryResponse["notice_rows"];
}

export interface RustProgressEvent {
  task_id: string;
  stage: string;
  percent: number;
  message: string;
}

export interface CommandResult<T> {
  ok: boolean;
  data?: T;
  message?: string;
}
