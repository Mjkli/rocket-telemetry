import { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";

function App() {
  let ws: WebSocket = new WebSocket("ws://127.0.0.1:8080");

  return (
    <>
      <div>
      </div>
    </>
  );
}

export default App;
