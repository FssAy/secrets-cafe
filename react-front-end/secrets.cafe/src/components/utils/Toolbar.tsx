import React from 'react';
import Bold from '@mui/icons-material/FormatBold';
import Italic from '@mui/icons-material/FormatItalic';
import Underlined from '@mui/icons-material/FormatUnderlined';

interface ToolbarProps {
    onBold: () => void;
    onItalic: () => void;
    onUnderline: () => void;
    onFontSize: (size: number) => void;
}

const Toolbar: React.FC<ToolbarProps> = ({
    onBold,
    onItalic,
    onUnderline,
    onFontSize,
}) => {
    return (
        <div>
            <Bold onClick={onBold}>Bold</Bold>
            <Italic onClick={onItalic}>Italic</Italic>
            <Underlined onClick={onUnderline}>Underline</Underlined>
            <select onChange={(e) => onFontSize(parseInt(e.target.value))}>
                <option value="12">12px</option>
                <option value="14">14px</option>
                <option value="16">16px</option>
            </select>
        </div>
    );
};

export default Toolbar;