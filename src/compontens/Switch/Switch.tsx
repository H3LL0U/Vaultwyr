

type SwitchProps = {
  checked?: boolean;
  onChange?: (checked: boolean) => void;
  disabled?: boolean;
  label?: string;
};

const Switch: React.FC<SwitchProps> = ({ checked = false, onChange = () => {}, disabled = false, label }) => {
  return (
    <label className="flex items-center space-x-3 cursor-pointer">
      {label && <span className="text-sm">{label}</span>}
      <div
        className={`relative inline-block w-12 h-6 transition duration-200 ease-in-out ${
          disabled ? "opacity-50 cursor-not-allowed" : ""
        }`}
      >
        <input
          type="checkbox"
          className="sr-only"
          checked={checked}
          onChange={(e) => onChange(e.target.checked)}
          disabled={disabled}
        />
        <span
          className={`block w-full h-full rounded-full transition-colors ${
            checked ? "bg-blue-500" : "bg-gray-300"
          }`}
        />
        <span
          className={`absolute left-1 top-1 w-4 h-4 bg-white rounded-full shadow transform transition-transform ${
            checked ? "translate-x-6" : ""
          }`}
        />
      </div>
    </label>
  );
};

export default Switch;