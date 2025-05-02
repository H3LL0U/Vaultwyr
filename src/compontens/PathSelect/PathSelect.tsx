import React from 'react';
import { open } from '@tauri-apps/plugin-dialog';

interface PathSelectProps {
  filename: string;
  setFileName: React.Dispatch<React.SetStateAction<string>>;
  submitFunc: (filename: string, setFileName: React.Dispatch<React.SetStateAction<string>>) => void;
}

const PathSelect: React.FC<PathSelectProps> = ({ submitFunc, filename, setFileName }) => {

  // Handle form submission
  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    submitFunc(filename, setFileName);
  };

  // Open the file dialog to choose a file
  const handleFileSelect = async () => {
    try {
      // Open a file selection dialog
      const selected = await open({
        multiple: false,  // Allow selecting only one file
        filters: [{ name: "Text Files", extensions: ["txt"] }],  // Filter to only show .txt files
      });

      // If a file is selected, update the filename state
      if (typeof selected === "string") {
        setFileName(selected);
      }
    } catch (error) {
      console.error("Error opening file dialog", error);
    }
  };

  return (
    <form className="row" onSubmit={onSubmit}>
      {/* Button to trigger file selection */}
      <button type="button" onClick={handleFileSelect}>
        Choose File
      </button>

      {/* Display the selected file or a default message if no file is selected */}
      <span style={{ marginLeft: "1rem", maxWidth: "300px", overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap" }}>
        {filename || "No file selected"}
      </span>

      {/* Submit button to trigger the submit function */}
      <button type="submit" style={{ marginLeft: "1rem" }}>
        Greet
      </button>
    </form>
  );
};

export default PathSelect;