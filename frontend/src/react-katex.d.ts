/**
 * Let TS handle Katex elements without complaining
 */

declare module 'react-katex' {
    import { ComponentType } from 'react';

    export interface KatexProps {
        math: string;
        errorColor?: string;
        renderError?: (error: Error) => React.ReactNode;
        settings?: any;
    }

    export const InlineMath: ComponentType<KatexProps>;
    export const BlockMath: ComponentType<KatexProps>;
}