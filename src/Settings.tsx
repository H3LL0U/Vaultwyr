import { useEffect, useState } from 'react'
import SettingsMenu from './compontens/SettingsMenu/SettingsMenu'
import SideBar from './compontens/SideBar/SideBar'
import ToastButton from './compontens/Toasts/ToastButton'
import ToastGroup from './compontens/Toasts/ToastGroup'
import SliderWithInput from './compontens/SliderWithInput/SliderWithInput'
import { AppSettings } from './API'
import API from './API'


function Settings() {
    
    const [selected_setting_index, setSelectedIndex] = useState(0) 
    const app_settings_promise = API.getSettings();
    


    
    const [appSettings, setAppSettings] = useState<AppSettings | null>(null);
    
    useEffect(() => {

        app_settings_promise.then((settings) => setAppSettings(settings));
        
    }, []);

    const possible_settings_tabs = [
        //General Settings
        <>
            <h3>Maximum file deletion size (GB)</h3>
            <SliderWithInput display_on_max='unlimited' min={0} max={500} onChange={(value)=>{
                let new_settings = appSettings;
                if (new_settings == null){
                    alert("Settings are loading")
                }
                else{
                    new_settings.MaxDeletionSize = value;
                    setAppSettings(new_settings)
                }
            }} initialValue={appSettings?.MaxDeletionSize}/>


        </>
        ,
        <></>
    ]

    function updateSettingsSidebar(index:number){
        setSelectedIndex(index)
    }

    function apply(){
    if (appSettings) {

      API.applySettings(appSettings);
      alert("Settings successfully applied!")
    } else {
      alert("Settings have not been loaded yet.");
    }
    }

  return <>
    <SettingsMenu>
      <div style={{ display: "flex", width: "100vw", height: "100vh" }}>
        <SideBar style={{ width: "33.33vw" }}>
          <ToastGroup onSet={updateSettingsSidebar}>
            <ToastButton >General Settings</ToastButton>
            <ToastButton >Extra Settings</ToastButton>
          </ToastGroup>
        </SideBar>

        <SideBar style={{ width: "66.66vw", backgroundColor:"#d6d4d4"}}>
            {possible_settings_tabs[selected_setting_index]}
            <button style={{ position: "absolute", bottom: "10px", right: "10px", width: "20%", backgroundColor:"#ebfff4"}} onClick={apply} >Apply</button>
        </SideBar>
      </div>
    </SettingsMenu>
    </>
  
}

export default Settings