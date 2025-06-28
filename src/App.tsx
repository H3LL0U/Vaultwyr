import { useState } from "react";

import PasswordSubmit from "./compontens/PasswordSubmit/PasswordSubmit";
import PathSelect from "./compontens/PathSelect/PathSelect";
import Popup from "reactjs-popup";
import { useEffect } from "react";

import API from "./API";


import ModeSelect from "./compontens/ModeSelect/ModeSelect";
import "reactjs-popup/dist/index.css";
import "./App.css";

function App() {

  const vaultwyr_extensions = ["vaultwyr", "fvaultwyr"]

  let [filename, setFileName] = useState("") //getting the filename/path selected by the user 

  const [openConfirmPopup, setOpenConfirmPopup] = useState(false); //getting the popup state
  
  const [selected_password,setSelectedPassword] = useState("") //getting the password written by the user
  const [repeat_password, setRepeatPassword] = useState("") // getting the repeated password
  



  const [_response, setResponse] = useState("") //response message after encrypting/decrypting

  let [mode, setMode] = useState(0) //getting the mode (0:encryption, 1: decryption)
  const messages = {
    popup_choose_password:
      "Choose the password that you want to use to encrypt your file(s)?",
    popup_confirm_encrypt: "Are you sure you want to ENCRYPT the file(s)?",
    popup_confirm_decrypt: "Are you sure you want to DECRYPT the file(s)?"
  };


function selectPath(path: string) {

  const is_vaultwyr_file = vaultwyr_extensions.some(ext => path.endsWith(ext)) ? 1 : 0;

  setMode(is_vaultwyr_file);
  mode = is_vaultwyr_file //change it immediately so that its value can be used in the next function (otherwise it is not going to update in time)
  setFileName(path);
  filename = path; //change it immediately so that its value can be used in the next function (otherwise it is not going to update in time)
  showConfirmPopUp();
}


  async function showConfirmPopUp() {
    //validate the selected path
    if (!await API.pathExists(filename)){
      alert("The path does not exist" + filename)
      return
    }

    let file_extension = filename.split(".").pop()?.trim() || ""
    if (vaultwyr_extensions.includes(file_extension) && mode==0){
      alert("Cannot encrypt a vaultwyr file twice")
      return
    }

    if (!(vaultwyr_extensions.includes(file_extension)) && mode == 1){
      alert("This file is not a vaultwyr file")
      return
    }

    if (filename){
      await setOpenConfirmPopup(true);
    }
    
  }
  
  function validatePassword () {
    setSelectedPassword(selected_password.trim())
    return (selected_password.length <32 && selected_password && ((mode===0 && repeat_password== selected_password) || mode !== 0))
  }
  
  async function confirmFileEncryption() { //AND DECRYPTION
    // Validate the password
    if (!validatePassword()) {
      const errorMessage = "Invalid password";
      setResponse(errorMessage);
      alert(errorMessage);
      return errorMessage;
    }
  
    //Ask for user confirmation
    const confirmed = await window.confirm("Are you sure you want to continue?");
    if (!confirmed) {
      const cancelMessage = "Operation cancelled by user.";
      setResponse(cancelMessage);
      return cancelMessage;
    }
  
    // Proceed with encryption or decryption
    let msg;
  
    if (mode === 0) {
      msg = await API.encryptWithPassword(selected_password, filename);
    } else if (mode === 1) {
      msg = await API.decryptWithPassword(selected_password, filename);
    }

    setFileName("")
    // Handle result
    if (msg && typeof msg === "string") {
      setResponse(msg);
      alert(msg);
      return msg;
    } else {
      const errorMessage = "Hmm, something went wrong.";
      setResponse(errorMessage);
      alert(errorMessage);
      return errorMessage;


      
    }
  }

  async function setup() {
    let args =  await API.getAppArgs()
    
    if (args.length >=2){
      selectPath(args[1])
    }
  }

  useEffect(() => {
    setup();
  }, []); 
  return (
    
    <main className="container">
      <p>{filename}</p>

      <ModeSelect index={mode} setIndex={setMode}><h1>Encrypt</h1><h1>Decrypt</h1></ModeSelect>

      <p className="selection_text">Choose file or a folder that you want to {mode ===0? "encrypt" : "decrypt"}</p>

      <PathSelect submitFunc={showConfirmPopUp} filename = {filename} setFileName = {setFileName}mode={mode}/>

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
              <p>{mode===0? messages.popup_confirm_encrypt: messages.popup_confirm_decrypt}</p>
            </div>
            <div className="actions">
              <PasswordSubmit password={selected_password} 
                setPassword={setSelectedPassword} 
                max_length={32} 
                mode={mode} 
                setRepeatPassword={setRepeatPassword}
                repeatPassword={repeat_password}>
              </PasswordSubmit>
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

      
    </main>
  );
}

export default App;