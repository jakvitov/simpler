import React, { Component, type ReactNode } from "react";
import ReportBugLink from "./ReportBugLink.tsx";

type Props = {
    children: ReactNode;
};

type State = {
    hasError: boolean;
};

/**
 * Common bugreport boundary to be used around basic elements
 */
class CommonErrorBoundary extends Component<Props, State> {
    constructor(props: Props) {
        super(props);
        this.state = { hasError: false };
    }

    static getDerivedStateFromError(_: Error): State {
        return { hasError: true };
    }

    componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
        console.error("Error caught by CommonErrorBoundary:", error, errorInfo);
    }

    render() {
        if (this.state.hasError) {
            return (<>
                <p>Unexpected error occurred. <ReportBugLink /></p>
            </>);
        }

        return this.props.children;
    }
}

export default CommonErrorBoundary;