import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import webSocket from "../../websocket/src/App.tsx";


function App() {
  let ws: WebSocket = webSocket("127.0.0.1:8080");

  return (
    <>
      <div>
      </div>
    </>
  );
}

export default App;
