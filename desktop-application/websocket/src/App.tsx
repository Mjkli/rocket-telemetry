const WebSocket = require('isomorphic-ws');

function Websocket() {
  const ws = new WebSocket("192.168.0.157:8081");
  ws.onopen = function open() {
    console.log('connected');
    ws.send(Date.now());
  };

  ws.onclose = function close() {
    console.log('disconnected');
  };

  ws.onmessage = function incoming(data) {
    console.log(`Roundtrip time: ${Date.now() - data.data} ms`);

    setTimeout(function timeout() {
      ws.send(Date.now());
    }, 500);
  };

}

export default Websocket;
