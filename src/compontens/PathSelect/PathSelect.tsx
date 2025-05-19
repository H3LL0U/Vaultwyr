import React from 'react';
import { open } from '@tauri-apps/plugin-dialog';

interface PathSelectProps {
  filename: string;
  setFileName: React.Dispatch<React.SetStateAction<string>>;
  submitFunc: (filename: string, setFileName: React.Dispatch<React.SetStateAction<string>>) => void;
  mode?: number;
}

const PathSelect: React.FC<PathSelectProps> = ({ submitFunc, filename, setFileName, mode = 0 }) => {
  const onSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    submitFunc(filename, setFileName);
  };

  const vaultExts = [".vaultwyr", ".fvaultwyr"];

  const handleFileSelect = async () => {
    try {
      const filters =
        mode === 1
          ? [{ name: "Vault Files", extensions: ["vaultwyr", "fvaultwyr"] }]
          : [{ name: "Non-Vault Files", extensions: ["*"] }];

      const selected = await open({
        multiple: false,
        directory: false,
        filters,
      });

      if (typeof selected === "string") {
        const lower = selected.toLowerCase();
        const isVault = vaultExts.some((ext) => lower.endsWith(ext));

        if ((mode === 1 && !isVault) || (mode === 0 && isVault)) {
          alert("This file type is not allowed in the current mode.");
          return;
        }

        setFileName(selected);
      }
    } catch (error) {
      console.error("Error selecting file", error);
    }
  };

  const handleFolderSelect = async () => {
    try {
      const selected = await open({
        multiple: false,
        directory: true,
      });

      if (typeof selected === "string") {
        setFileName(selected);
      }
    } catch (error) {
      console.error("Error selecting folder", error);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFileName(e.target.value);
  };

  return (
    <form className="row" onSubmit={onSubmit}>
      <button type="button" onClick={handleFileSelect}>
        📄
      </button>

      <button 
  type="button" 
  onClick={handleFolderSelect} 
  style={{ 
    marginLeft: "0.5rem", 
    display: mode === 1 ? "none" : "block" 
  }}
>
        📁
      </button>

      <input
        style={{
          marginLeft: "1rem",
          maxWidth: "300px",
          overflow: "hidden",
          textOverflow: "ellipsis",
          whiteSpace: "nowrap",
        }}
        placeholder="path/to/file-or-folder"
        value={filename}
        onChange={handleChange}
      />

      <button type="submit" style={{ marginLeft: "1rem" }}>
        Confirm
      </button>
    </form>
  );
};

export default PathSelect;