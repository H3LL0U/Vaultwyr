interface PasswordSubmitProps {
    password: string;
    setPassword: (password: string) => void;
    onSubmit?: () => void;
    with_button?:boolean; 
    max_length?:number;
  }
  
  const PasswordSubmit = ({ password, setPassword, onSubmit ,with_button,max_length}: PasswordSubmitProps) => (
    <div style={{ display: "flex", flexDirection: "column", gap: "0.5rem" }}>
      <label htmlFor="password">Enter Password:</label>
      <input
        id="password"
        type="password"
        value={password}
        maxLength={max_length}
        onChange={(e) => setPassword(e.target.value)}
        style={{ padding: "0.5rem" }
        
    }
        
      />
        
            {with_button? <button onClick={onSubmit}>Submit</button>:<></>}

        
      
    </div>
  );
export default PasswordSubmit