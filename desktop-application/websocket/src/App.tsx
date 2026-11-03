import Websocket from 'isomorphic-ws';


function Websocket() {
  const ws = new WebSocket("192.168.0.157:8081");
  ws.onopen = function open() {
    console.log('connected');
  };

  ws.onclose = function close() {
    console.log('disconnected');
  };

  ws.onmessage = function incoming(data) {
    console.log("Received: " + data.data);
  };

}

export default Websocket;
