import React, { useState, useEffect } from "react";

interface SliderWithInputProps {
  min?: number;
  max?: number;
  step?: number;
  initialValue?: number;
  onChange?: (value: number) => void;
  display_on_max?: string;
}

const SliderWithInput: React.FC<SliderWithInputProps> = ({
  min = 0,
  max = 200,
  step = 1,
  initialValue = 50,
  onChange,
  display_on_max,
}) => {
  const clamp = (val: number, min: number, max: number) => Math.min(Math.max(val, min), max);

  const [inputValue, setInputValue] = useState<string>(initialValue.toString());

  // Compute numericValue: if input is empty, treat as 0; clamp within range
  const numericValue = inputValue === "" ? 0 : clamp(Number(inputValue), min, max);

  useEffect(() => {
    setInputValue(initialValue.toString());
  }, [initialValue]);

  const handleSliderChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = Number(e.target.value);
    setInputValue(newValue.toString());
    onChange?.(newValue);
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const val = e.target.value;
    if (val === "") {
      setInputValue("");
      onChange?.(0);
      return;
    }
    const newValue = Number(val);
    if (!isNaN(newValue)) {
      if (newValue >= min && newValue <= max) {
        setInputValue(val);
        onChange?.(newValue);
      }
    }
  };

  // Show display_on_max string if value equals max and display_on_max is provided
  const inputDisplayValue =
    display_on_max && numericValue === max ? display_on_max : inputValue;

  return (
    <>
      <style>{`
        input[type="range"] {
          -webkit-appearance: none;
          width: 100%;
          margin: 0;
          padding: 0;
          background: transparent;
        }
        input[type="range"]:focus {
          outline: none;
        }
        input[type="range"]::-webkit-slider-runnable-track {
          height: 6px;
          background: #ddd;
          border-radius: 3px;
        }
        input[type="range"]::-moz-range-track {
          height: 6px;
          background: #ddd;
          border-radius: 3px;
        }
        input[type="range"]::-webkit-slider-thumb {
          -webkit-appearance: none;
          appearance: none;
          margin-top: -6px;
          width: 18px;
          height: 18px;
          background: #333;
          border-radius: 50%;
          cursor: pointer;
          border: none;
          box-shadow: 0 0 2px rgba(0,0,0,0.5);
          transition: background 0.3s ease;
          position: relative;
          z-index: 1;
        }
        input[type="range"]::-moz-range-thumb {
          width: 18px;
          height: 18px;
          background: #333;
          border-radius: 50%;
          border: none;
          cursor: pointer;
          box-shadow: 0 0 2px rgba(0,0,0,0.5);
          transition: background 0.3s ease;
          position: relative;
          z-index: 1;
        }
        input[type="range"]:hover::-webkit-slider-thumb {
          background: #555;
        }
        input[type="range"]:hover::-moz-range-thumb {
          background: #555;
        }
      `}</style>
      <div style={{ display: "flex", alignItems: "center", gap: "0.5rem" }}>
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={numericValue}
          onChange={handleSliderChange}
          style={{ flex: 1 }}
        />
        <input
          type="text" // use text so we can show string like display_on_max
          value={inputDisplayValue}
          onChange={handleInputChange}
          style={{ width: "4rem" }}
          // optionally restrict input with pattern to numbers or the display_on_max string
          pattern={display_on_max ? `^\\d*$|^${display_on_max}$` : undefined}
        />
      </div>
    </>
  );
};

export default SliderWithInput;

