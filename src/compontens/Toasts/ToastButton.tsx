import React, { ReactNode, CSSProperties } from "react";

export interface ToastButtonProps {
  children?: ReactNode;
  onClick?: () => void;
  style?: CSSProperties;
  active?: boolean; // optional active state
}

const ToastButton: React.FC<ToastButtonProps> = ({
  children,
  onClick,
  style = {},
  active = false,
}) => {
  // Lighter blue colors for active state
  const activeBg = "#3b7ddd";       // lighter blue (active)
  const activeHoverBg = "#2a5cb8";  // slightly darker blue on hover (active)
  const activeBoxShadow = "0 0 8px rgba(59, 125, 221, 0.7)";

  return (
    <button
      onClick={onClick}
      style={{
        width: "100%",
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        gap: "0.5rem",
        padding: "0.5rem 1rem",
        margin: "0.25rem 0",
        borderRadius: "8px",
        border: "none",
        cursor: "pointer",
        textAlign: "center",
        backgroundColor: active ? activeBg : "#e0e0e0",
        color: active ? "white" : "black",
        boxShadow: active
          ? activeBoxShadow
          : "0 1px 3px rgba(0,0,0,0.1)",
        transition: "background-color 0.3s ease, box-shadow 0.3s ease",
        ...style,
      }}
      onMouseEnter={(e) => {
        e.currentTarget.style.backgroundColor = active
          ? activeHoverBg
          : "#d5d5d5";
      }}
      onMouseLeave={(e) => {
        e.currentTarget.style.backgroundColor = active
          ? activeBg
          : "#e0e0e0";
      }}
    >
      {children}
    </button>
  );
};

export default ToastButton;
