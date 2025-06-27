import React, { ReactNode, CSSProperties } from "react";

interface SideBarProps {
  width?: string; // CSS width, default "33.33vw" (1/3 viewport width)
  style?: CSSProperties; // Optional extra styles to merge
  children?: ReactNode;
}

const SideBar: React.FC<SideBarProps> = ({
  width = "33.33vw",
  style = {},
  children,
}) => {
  return (
    <div
      style={{
        position: "relative",
        height: "90vh",
        left: 0,
        top: 0,
        bottom: 0,
        width,
        backgroundColor: "#f0f0f0",
        
        
        
        padding: "1rem",
        boxShadow: "2px 0 5px rgba(0,0,0,0.1)",
        overflowX: "auto",
        ...style,
      }}
    >
      {children}
    </div>
  );
};

export default SideBar;