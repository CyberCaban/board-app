import { getCookie } from "./utils";

class WS {
  private ws: WebSocket;
  private url: string;
  constructor(url: string, setFn: (msg: any) => void, conversation_id: string) {
    this.url = url;
    this.ws = new WebSocket(url);
    this.ws.onmessage = (e) => {
      const data = JSON.parse(e.data);
      setFn(data);
    };
    this.ws.onopen = () => {
      this.ws.send(
        JSON.stringify({ token: getCookie("token") || "", conversation_id }),
      );
      console.log("Connected to server");
    };
    this.ws.onclose = () => {
      console.log("Disconnected from server");
    };
    this.ws.onerror = (e) => {
      console.log("Error: ", e);
      this.ws.close();
    };
  }
  send(data: any) {
    this.ws.send(JSON.stringify(data));
  }
  close() {
    this.ws.close();
  }
  get getUrl(): string {
    return this.url;
  }
}

export default WS;
