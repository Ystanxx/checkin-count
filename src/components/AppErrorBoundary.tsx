import { Component, type ErrorInfo, type ReactNode } from "react";

interface AppErrorBoundaryProps {
  children: ReactNode;
}

interface AppErrorBoundaryState {
  hasError: boolean;
  message: string;
}

export class AppErrorBoundary extends Component<
  AppErrorBoundaryProps,
  AppErrorBoundaryState
> {
  state: AppErrorBoundaryState = {
    hasError: false,
    message: "",
  };

  static getDerivedStateFromError(error: Error): AppErrorBoundaryState {
    return {
      hasError: true,
      message: error.message,
    };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("应用渲染失败", error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <main className="fatal-screen">
          <section className="fatal-card">
            <p className="fatal-kicker">界面初始化失败</p>
            <h1>请重新启动应用</h1>
            <p>
              当前已拦截前端运行时异常，避免继续显示空白页面。
            </p>
            <pre>{this.state.message || "未捕获到具体错误信息。"}</pre>
          </section>
        </main>
      );
    }

    return this.props.children;
  }
}
