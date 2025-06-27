import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Settings from "./Settings";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Settings></Settings>
    <App />
  </React.StrictMode>,
);
