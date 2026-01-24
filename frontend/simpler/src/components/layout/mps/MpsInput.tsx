import { useState, useRef, useEffect } from 'react';

interface MPSInputProps {
    value?: string;
    onChange?: (value: string) => void;
    placeholder?: string;
    rows?: number;
}

function MPSInput({
                      value = '',
                      onChange,
                      placeholder = 'Enter MPS code...',
                      rows = 10
                  }: MPSInputProps) {
    const [text, setText] = useState(value);
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const highlightRef = useRef<HTMLPreElement>(null);

    // MPS keywords
    const keywords = [
        'NAME', 'ROWS', 'COLUMNS', 'RHS', 'BOUNDS', 'RANGES', 'ENDATA',
        'MIN', 'MAX', 'E', 'L', 'G', 'N',
        'LO', 'UP', 'FX', 'FR', 'MI', 'PL', 'BV', 'LI', 'UI', 'SC'
    ];

    const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const newValue = e.target.value;
        setText(newValue);
        onChange?.(newValue);
    };

    const highlightSyntax = (code: string): string => {
        if (!code) return '';

        // Escape HTML
        let highlighted = code
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;');

        // Highlight keywords (must be at start of line or after whitespace)
        keywords.forEach(keyword => {
            const regex = new RegExp(`(^|\\s)(${keyword})(?=\\s|$)`, 'gm');
            highlighted = highlighted.replace(
                regex,
                '$1<span style="color: #0066cc; font-weight: bold;">$2</span>'
            );
        });

        // Highlight numbers
        highlighted = highlighted.replace(
            /\b(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?\b/g,
            '<span style="color: #098658;">$&</span>'
        );

        // Highlight comments (lines starting with *)
        highlighted = highlighted.replace(
            /^(\s*\*.*)$/gm,
            '<span style="color: #6a737d; font-style: italic;">$1</span>'
        );

        return highlighted;
    };

    // Sync scroll
    const handleScroll = () => {
        if (textareaRef.current && highlightRef.current) {
            highlightRef.current.scrollTop = textareaRef.current.scrollTop;
            highlightRef.current.scrollLeft = textareaRef.current.scrollLeft;
        }
    };

    useEffect(() => {
        setText(value);
    }, [value]);

    return (
        <div style={{ position: 'relative', width: '100%' }}>
            {/* Syntax highlighted background */}
            <pre
                ref={highlightRef}
                style={{
                    position: 'absolute',
                    top: 0,
                    left: 0,
                    right: 0,
                    bottom: 0,
                    margin: 0,
                    padding: '0.375rem 0.75rem',
                    backgroundColor: '#F5F5F5',
                    fontSize: '1rem',
                    fontFamily: 'monospace',
                    lineHeight: '1.5',
                    overflow: 'auto',
                    pointerEvents: 'none',
                    whiteSpace: 'pre-wrap',
                    wordWrap: 'break-word',
                    zIndex: 1,
                }}
                dangerouslySetInnerHTML={{ __html: highlightSyntax(text) }}
            />

            {/* Transparent textarea */}
            <textarea
                ref={textareaRef}
                value={text}
                onChange={handleChange}
                onScroll={handleScroll}
                placeholder={placeholder}
                rows={rows}
                spellCheck={false}
                style={{
                    position: 'relative',
                    width: '100%',
                    backgroundColor: 'transparent',
                    color: 'transparent',
                    fontSize: '1rem',
                    fontFamily: 'monospace',
                    lineHeight: '1.5',
                    border: '0px',
                    padding: '0.375rem 0.75rem',
                    resize: 'vertical',
                    caretColor: 'black',
                    zIndex: 2,
                    WebkitTextFillColor: 'transparent',
                }}
            />
        </div>
    );
}

// Demo component
export default function Demo() {
    const [code, setCode] = useState(`NAME          EXAMPLE
ROWS
 N  OBJ
 L  C1
 G  C2
COLUMNS
    X1        OBJ       -1
    X1        C1        1
    X1        C2        2
    X2        OBJ       -2
    X2        C1        1
    X2        C2        1
RHS
    RHS1      C1        4
    RHS1      C2        3
BOUNDS
 LO BND1      X1        0
 UP BND1      X1        10
 FR BND1      X2
ENDATA`);

    return (
        <div style={{maxWidth: '800px', margin: '0 auto' }}>
            <MPSInput
                value={code}
                onChange={setCode}
                rows={15}
            />
        </div>
    );
}