use crate::application::dto::{BuildSummaryResponse, NoticeBuildResponse, PreviewResponse};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct AppState {
    inner: Arc<Mutex<AppRuntimeState>>,
}

#[derive(Debug, Default)]
pub struct AppRuntimeState {
    pub preview: Option<PreviewResponse>,
    pub summary: Option<BuildSummaryResponse>,
    pub notice: Option<NoticeBuildResponse>,
}

impl AppState {
    pub fn read<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&AppRuntimeState) -> T,
    {
        let guard = self.inner.lock().expect("state poisoned");
        f(&guard)
    }

    pub fn write<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut AppRuntimeState) -> T,
    {
        let mut guard = self.inner.lock().expect("state poisoned");
        f(&mut guard)
    }
}
