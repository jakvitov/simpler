import { useState, useRef, useEffect } from 'react';
import { LAST_MPS_INPUT_DATA } from "../../../utils/storageConstants.ts";

interface MPSInputProps {
    value?: string;
    onChange?: (value: string) => void;
    placeholder?: string;
    rows?: number;
}

export default function MPSInput({
                                     value = '',
                                     onChange,
                                     placeholder = 'Enter MPS code...',
                                     rows = 10
                                 }: MPSInputProps) {
    const [text, setText] = useState(value);
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const highlightRef = useRef<HTMLPreElement>(null);

    const keywords = [
        'NAME', 'ROWS', 'COLUMNS', 'RHS', 'BOUNDS', 'RANGES', 'ENDATA',
        'MIN', 'MAX', 'E', 'L', 'G', 'N',
        'LO', 'UP', 'FX', 'FR', 'MI', 'PL', 'BV', 'LI', 'UI', 'SC'
    ];

    const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const newValue = e.target.value;
        setText(newValue);
        try {
            localStorage.setItem(LAST_MPS_INPUT_DATA, newValue);
        } catch (e) {
            localStorage.removeItem(LAST_MPS_INPUT_DATA);
        }
        onChange?.(newValue);
    };

    const highlightSyntax = (code: string): string => {
        if (!code) return '';

        let highlighted = code
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;');

        keywords.forEach(keyword => {
            const regex = new RegExp(`(^|\\s)(${keyword})(?=\\s|$)`, 'gm');
            highlighted = highlighted.replace(
                regex,
                '$1<span style="color: #0066cc; font-weight: bold;">$2</span>'
            );
        });

        highlighted = highlighted.replace(
            /\b(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?\b/g,
            '<span style="color: #098658;">$&</span>'
        );

        highlighted = highlighted.replace(
            /^(\s*\*.*)$/gm,
            '<span style="color: #6a737d; font-style: italic;">$1</span>'
        );

        return highlighted + '\n';
    };

    const handleScroll = () => {
        if (textareaRef.current && highlightRef.current) {
            highlightRef.current.scrollTop = textareaRef.current.scrollTop;
            highlightRef.current.scrollLeft = textareaRef.current.scrollLeft;
        }
    };

    const computedRows = Math.max(rows, text.split('\n').length);

    useEffect(() => {
        setText(value);
    }, [value]);

    const sharedStyle: React.CSSProperties = {
        margin: 0,
        padding: '0.375rem 0.75rem',
        fontSize: '1rem',
        fontFamily: 'monospace',
        lineHeight: '1.5',
        whiteSpace: 'pre',
        wordWrap: 'normal',
        overflowWrap: 'normal',
        tabSize: 4,
    };

    return (
        <div style={{ position: 'relative', width: '100%', overflow: 'hidden' }}>
            {/* Syntax highlighted background */}
            <pre
                ref={highlightRef}
                aria-hidden="true"
                style={{
                    ...sharedStyle,
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 0,
                    backgroundColor: '#F5F5F5',
                    overflow: 'auto',
                    pointerEvents: 'none',
                    zIndex: 1,
                }}
                dangerouslySetInnerHTML={{ __html: highlightSyntax(text) }}
            />

            {/* Transparent textarea — rows drives the height for both layers */}
            <textarea
                ref={textareaRef}
                value={text}
                onChange={handleChange}
                onScroll={handleScroll}
                placeholder={placeholder}
                rows={computedRows}
                wrap="off"
                spellCheck={false}
                style={{
                    ...sharedStyle,
                    position: 'relative',
                    display: 'block',
                    width: '100%',
                    backgroundColor: 'transparent',
                    color: 'transparent',
                    border: '0px',
                    resize: 'none',
                    caretColor: 'black',
                    overflow: 'auto',
                    zIndex: 2,
                    WebkitTextFillColor: 'transparent',
                    boxSizing: 'border-box',
                }}
            />
        </div>
    );
}