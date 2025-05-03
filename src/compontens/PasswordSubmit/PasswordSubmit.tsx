

interface PasswordSubmitProps {
  password: string;
  setPassword: (password: string) => void;
  repeatPassword?: string;
  setRepeatPassword?: (password: string) => void;
  onSubmit?: () => void;
  with_button?: boolean;
  max_length?: number;
  mode?: number; // if a new password will check will add an extra field where the password needs to be repeated
}

const PasswordSubmit = ({
  password,
  setPassword,
  repeatPassword,
  setRepeatPassword,
  onSubmit,
  with_button,
  max_length,
  mode,
}: PasswordSubmitProps) => (
  <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
    <label htmlFor="password">Enter Password:</label>
    <input
      id="password"
      type="password"
      value={password}
      maxLength={max_length}
      onChange={(e) => setPassword(e.target.value)}
      style={{ padding: "0.5rem" }}
    />

    {mode === 0 && repeatPassword !== undefined && setRepeatPassword ? (
      <>
        <label htmlFor="repeat-password">Repeat Password:</label>
        <input
          id="repeat-password"
          type="password"
          value={repeatPassword}
          maxLength={max_length}
          onChange={(e) => setRepeatPassword(e.target.value)}
          style={{ padding: "0.5rem" }}
        />
      </>
    ) : null}

    {with_button ? <button onClick={onSubmit}>Submit</button> : null}
  </div>
);

export default PasswordSubmit;