
function websocket(addr:  string) {
  let ws: WebSocket = new WebSocket("ws://" + addr);
  
  ws.onopen = function open() {
    console.log('connected');
  };

  ws.onclose = function close() {
    console.log('disconnected');
  };

  ws.onmessage = function incoming(data) {
    console.log("Received: " + data.data);
  };

  return ws;

}

export default websocket;
