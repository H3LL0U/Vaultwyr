import React, { useState } from "react";

interface SettingsMenuProps {
  children?: React.ReactNode;
}

const SettingsMenu: React.FC<SettingsMenuProps> = ({ children }) => {
  const [open, setOpen] = useState(false);

  return (
    <>
      {/* Button to toggle menu */}
      <button
        onClick={() => setOpen(!open)}
        style={{
          position: "fixed",
          top: 20,
          right: open ? -30 : 20, // fully visible when closed, partially offscreen when open
          zIndex: 21,
          width: 50,
          height: 50,
          
          border: "none",
          
          color: "black",
          cursor: "pointer",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          fontSize: "1.5rem",
          transition: "right 0.4s ease-in-out",
          userSelect: "none",
          boxShadow: "0 0 5px rgba(0,0,0,0.3)",
        }}
        aria-label={open ? "Close settings menu" : "Open settings menu"}
        title={open ? "Close settings menu" : "Open settings menu"}
      >
        {open ? "⬅️" : "⚙️"}
      </button>

      {/* Sliding menu */}
      <div
        style={{
          position: "fixed",
          top: 0,
          left: open ? "0" : "-100vw",
          width: "100vw",
          height: "100vh",
          backgroundColor: "white",
          boxShadow: "0 0 10px rgba(0,0,0,0.3)",
          transition: "left 0.4s ease-in-out",
          zIndex: 20,
          overflow: "auto",
        }}
      >
        <div style={{ textAlign: "center" }}>{children}</div>
      </div>
    </>
  );
};

export default SettingsMenu;
