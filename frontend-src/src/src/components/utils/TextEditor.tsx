import React, { useState } from 'react';
import RichTextEditor, { ToolbarConfig } from 'react-rte';

const TextEditor: React.FC = () => {
  const [value, setValue] = useState(RichTextEditor.createEmptyValue());

  const onChange = (value: any) => {
    setValue(value);
  };

  const toolbarConfig: ToolbarConfig = {
    display: [
      "INLINE_STYLE_BUTTONS",
      "BLOCK_TYPE_BUTTONS",
      "BLOCK_ALIGNMENT_BUTTONS",
      "BLOCK_TYPE_DROPDOWN",
    ],
    INLINE_STYLE_BUTTONS: [
      { label: "Bold", style: "BOLD" },
      { label: "Italic", style: "ITALIC" },
      { label: "Underline", style: "UNDERLINE" },
    ],
    BLOCK_TYPE_BUTTONS: [
      { label: "UL", style: "unordered-list-item" },
      { label: "OL", style: "ordered-list-item" },
    ],
    BLOCK_ALIGNMENT_BUTTONS: [
      { label: "Align Left", style: "ALIGN_LEFT" },
      { label: "Align Center", style: "ALIGN_CENTER" },
      { label: "Align Right", style: "ALIGN_RIGHT" },
      { label: "Align Justify", style: "ALIGN_JUSTIFY" },
    ],
    BLOCK_TYPE_DROPDOWN: [
      { label: "Normal", style: "unstyled" },
      { label: "Heading Large", style: "header-one" },
      { label: "Heading Medium", style: "header-two" },
      { label: "Heading Small", style: "header-three" },
    ],
  };

  return (
    <div>
      <RichTextEditor
        value={value}
        onChange={onChange}
        toolbarConfig={toolbarConfig}
        className="w-full min-h-[400px] min-w-[400px] border border-gray-300 rounded-md p-4 mt-4 focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
    </div>
  );
};

export default TextEditor;