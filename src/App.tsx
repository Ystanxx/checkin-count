import { AppErrorBoundary } from "./components/AppErrorBoundary";
import { AppShell } from "./components/AppShell";

export function App() {
  return (
    <AppErrorBoundary>
      <AppShell />
    </AppErrorBoundary>
  );
}
