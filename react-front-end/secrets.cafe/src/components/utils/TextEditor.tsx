import React, { useState } from 'react';
import Toolbar from './Toolbar';

const TextEditor: React.FC = () => {
    const [text, setText] = useState('');
    const [isBold, setIsBold] = useState(false);
    const [isItalic, setIsItalic] = useState(false);
    const [isUnderline, setIsUnderline] = useState(false);
    const [fontSize, setFontSize] = useState(16);

    const handleBold = () => {
        setIsBold((prev) => !prev);
    };

    const handleItalic = () => {
        setIsItalic((prev) => !prev);
    };

    const handleUnderline = () => {
        setIsUnderline((prev) => !prev);
    };

    const handleFontSize = (size: number) => {
        setFontSize(size);
    };

    return (
        <div>
            <Toolbar
                onBold={handleBold}
                onItalic={handleItalic}
                onUnderline={handleUnderline}
                onFontSize={handleFontSize}
            />
            <textarea
                value={text}
                onChange={(e) => setText(e.target.value)}
                style={{
                    fontWeight: isBold ? 'bold' : 'normal',
                    fontStyle: isItalic ? 'italic' : 'normal',
                    textDecoration: isUnderline ? 'underline' : 'none',
                    fontSize: `${fontSize}px`,
                }}
                className="flex min-h-[80px] w-full rounded-base border-2 font-bold border-black bg-white px-3 py-2 text-sm ring-offset-white placeholder:text-black/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-slate-950 focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
            />
        </div>
    );
};

export default TextEditor;