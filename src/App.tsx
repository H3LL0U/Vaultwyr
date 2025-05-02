import { useState } from "react";

import PasswordSubmit from "./compontens/PasswordSubmit/PasswordSubmit";
import PathSelect from "./compontens/PathSelect/PathSelect";
import Popup from "reactjs-popup";
import encryptWithPassword from "./API";
import "reactjs-popup/dist/index.css";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [filename, setFileName] = useState("")
  const [openConfirmPopup, setOpenConfirmPopup] = useState(false);

  const [selected_password,setSelectedPassword] = useState("")
  const [response, setResponse] = useState("")
  const messages = {
    popup_choose_password:
      "Choose the password that you want to use to encrypt your file(s)",
    popup_confirm: "Are you sure you want to encrypt the files?",
  };

  function showConfirmPopUp() {
    if (filename){
      setOpenConfirmPopup(true);
    }
    
  }
  
  function validatePassword () {
    setSelectedPassword(selected_password.trim())
    return (selected_password.length <32 && selected_password)
  }

  async function confirmFileEncryption(){
    //TODO: Add an extra confirmation step
    if (validatePassword()){
      const msg= await encryptWithPassword(selected_password,filename)
      if (typeof msg === "string"){
        setResponse(msg)
        alert(msg)
        return msg
      }
      else {
        setResponse("hmm")
        return "none"
      }
    }

  }
  return (
    
    <main className="container">
      <p>{response}</p>
      <h1>Welcome to Tauri + React</h1>

      <p className="selection_text">Choose a file that you want to encrypt</p>

      <PathSelect submitFunc={showConfirmPopUp} filename = {filename} setFileName = {setFileName} />

      {/* Controlled Popup using render function */}
      <Popup
        open={openConfirmPopup}
        modal
        onClose={() => setOpenConfirmPopup(false)}
      >
        {((close: () => void) => (
          <div className="modal" >
            <button className="close" onClick={close}>
              &times;
            </button>
            <div className="header">Confirmation</div>
            <div className="content">
              <p>{messages.popup_confirm}</p>
            </div>
            <div className="actions">
              <PasswordSubmit password={selected_password} setPassword={setSelectedPassword} ></PasswordSubmit>
              <div className="flex">
              <button
                className="button selected"
                onClick={ async () => {
                  
                  await confirmFileEncryption()
                  close();
                }}
              >
                Confirm
              </button>
              <button className="button unselected" onClick={close}>
                Cancel
              </button>
              </div>
            </div>
          </div>
        )) as any}
      </Popup>

      <p>{greetMsg}</p>
    </main>
  );
}

export default App;