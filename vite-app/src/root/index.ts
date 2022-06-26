// Wasm
// (vite eklentisi tarafından otomatik olarak
// eklendiği için package.json dosyasına bulunamıyor.)
// @ts-ignore-start
import init, { /*init_logger,*/ run_app } from "yew-app";
// @ts-ignore-end

// Css
import "./index.css";

// Wasm çalıştır
init().then(() => {
  run_app();
});
