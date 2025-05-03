import React, { useState, } from 'react';
import './ModeSelect.css';

interface ModeSelectProps {
    children: React.ReactNode[];
    index: number;
    setIndex: React.Dispatch<React.SetStateAction<number>>;
  }
const ModeSelect: React.FC<ModeSelectProps> = ({ children, index, setIndex}:ModeSelectProps) => {
  
  const [prevIndex, setPrevIndex] = useState(0);
  const [direction, setDirection] = useState<'left' | 'right'>('right');

  const total = React.Children.count(children);

  const move = (dir: 'left' | 'right') => {
    setDirection(dir);
    setPrevIndex(index);
    if (dir === 'right') {
      setIndex((prev) => (prev + 1) % total);
    } else {
      setIndex((prev) => (prev - 1 + total) % total);
    }
  };

  return (
    <div className="mode-select">
      <button onClick={() => move('left')}>&lt;</button>
      <div className="mode-container">
        <div className={`mode-item exit-${direction}`} key={prevIndex}>
          {children[prevIndex]}
        </div>
        <div className={`mode-item enter-${direction}`} key={index}>
          {children[index]}
        </div>
      </div>
      <button onClick={() => move('right')}>&gt;</button>
    </div>
  );
};

export default ModeSelect;