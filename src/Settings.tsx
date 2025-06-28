import { useEffect, useState } from 'react'
import SettingsMenu from './compontens/SettingsMenu/SettingsMenu'
import SideBar from './compontens/SideBar/SideBar'
import ToastButton from './compontens/Toasts/ToastButton'
import ToastGroup from './compontens/Toasts/ToastGroup'
import SliderWithInput from './compontens/SliderWithInput/SliderWithInput'
import { AppSettings } from './API'
import API from './API'
import Switch from './compontens/Switch/Switch'

function Settings() {
    
    const [selected_setting_index, setSelectedIndex] = useState(0) 
    const app_settings_promise = API.getSettings();
    
    


    
    const [appSettings, setAppSettings] = useState<AppSettings | null>(null);
    
    useEffect(() => {

        app_settings_promise.then((settings) => setAppSettings(settings));
        
    }, []);

    const possible_settings_tabs = [
        //Encryption Settings
        <>
            <h3>Maximum file deletion size (GB)</h3>
            <SliderWithInput display_on_max='unlimited' min={0} max={500} initialValue={appSettings?.MaxDeletionSize} onChange={(value)=>{
                let new_settings = appSettings;
                if (new_settings == null){
                    alert("Settings are loading")
                }
                else{
                    
                    new_settings.MaxDeletionSize = value;
                    setAppSettings(new_settings)
                }
            }} />
            <br />

            <h3>File chunk size (bytes)</h3> 
            <h4>
              Equals to around{" "}
              {appSettings?.ChunkSize !== undefined
                ? (appSettings.ChunkSize / (1024 * 1024)).toFixed(2)
                : "Unknown"}{" "}
              Megabytes
            </h4>
            <SliderWithInput min={256} max={64_000_000} initialValue={appSettings?.ChunkSize} onChange={(value) => {
              if (appSettings == null) {
                alert("Settings are loading");
              } else {
                setAppSettings({
                  ...appSettings,
                  ChunkSize: value,
                });
              }
            }}/>


        </>
        ,
        //Decryption Settings
        <>
          <h3>Restore to original path</h3>
            <Switch onChange={(value) => {
              if (!appSettings) {
                alert("Settings are loading");
                return;
              }
              setAppSettings({
                ...appSettings,
                RestoreToOriginalFolder: value
              });
            }} checked = {appSettings?.RestoreToOriginalFolder}></Switch>

          <br />
        
        </>
    ]



    function apply(){
    if (appSettings) {

      API.applySettings(appSettings);
      
    } else {
      alert("Settings have not been loaded yet.");
    }
    }

  return <>
    <SettingsMenu>
      <div style={{ display: "flex", width: "100vw", height: "100vh" }}>
        <SideBar style={{ width: "33.33vw" }}>
          <ToastGroup onSet={setSelectedIndex}>
            <ToastButton >Encryption Settings</ToastButton>
            <ToastButton >Decryption Settings</ToastButton>
          </ToastGroup>
        </SideBar>

        <SideBar style={{ width: "66.66vw", backgroundColor:"#d6d4d4"}}>
            {possible_settings_tabs.map((tab, index) => (
              <div
                key={index}
                style={{ display: index === selected_setting_index ? "block" : "none" }}
              >
                {tab}
              </div>
            ))
            //only display the selected tab
            } 
            <button style={{ position: "absolute", bottom: "10px", right: "10px", width: "20%", backgroundColor:"#ebfff4"}} onClick={apply} >Apply</button>
        </SideBar>
      </div>
    </SettingsMenu>
    </>
  
}

export default Settings