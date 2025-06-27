import React, { ReactElement, useState, Children, isValidElement } from "react";
import { ToastButtonProps } from "./ToastButton";

interface ToastGroupProps {
  children: ReactElement<ToastButtonProps> | ReactElement<ToastButtonProps>[];
  onSet?: (index: number) => void;  // setter callback with selected index
}
const ToastGroup: React.FC<ToastGroupProps> = ({ children ,onSet = (_)=>{}}) => {
  const [activeIndex, setActiveIndex] = useState<number | null>(null);

  // Normalize children to array
  const childArray = Children.toArray(children).filter(isValidElement) as ReactElement<ToastButtonProps>[];

  return (
    <div style={{ display: "flex", flexDirection: "column", width: "100%" }}>
      {childArray.map((child, index) =>
        React.cloneElement(child, {
          active: index === activeIndex,
          onClick: () => {
            setActiveIndex(index)
            onSet(index)
          },
          key: child.key || index,
        })
      )}
    </div>
  );
};

export default ToastGroup;