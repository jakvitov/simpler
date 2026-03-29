
import { Component, type ErrorInfo, type ReactNode } from "react";

interface Props {
    children: ReactNode;
}

interface State {
    hasError: boolean;
}

/**
 * Top level bugreport boundary used around App
 */
class TopLevelErrorBoundary extends Component<Props, State> {
    state: State = { hasError: false };

    static getDerivedStateFromError(): State {
        return { hasError: true };
    }

    componentDidCatch(error: Error, info: ErrorInfo) {
        console.error("Unhandled bugreport:", error, info.componentStack);
        alert(`🐞 An unexpected error occurred: ${error.message}`);
        window.location.href = "/report-bug";
    }

    render() {
        if (this.state.hasError) {
            return null;
        }
        return this.props.children;
    }
}

export default TopLevelErrorBoundary;