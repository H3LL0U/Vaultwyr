import React, { useState, useEffect } from "react";
import styles from "./SliderWithInput.module.css";

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

  const inputDisplayValue =
    display_on_max && numericValue === max ? display_on_max : inputValue;

  return (
    <div className={styles.wrapper}>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={numericValue}
        onChange={handleSliderChange}
        className={styles.slider}
      />
      <input
        type="text"
        value={inputDisplayValue}
        onChange={handleInputChange}
        className={styles.textInput}
        pattern={display_on_max ? `^\\d*$|^${display_on_max}$` : undefined}
      />
    </div>
  );
};

export default SliderWithInput;
