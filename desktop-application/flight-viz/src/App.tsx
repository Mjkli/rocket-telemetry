import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import webSocket from "../../websocket/src/App.tsx";


function App() {
  let ws: WebSocket = webSocket("192.168.0.118:8081");

  return (
    <>
      <div>
      </div>
    </>
  );
}

export default App;
