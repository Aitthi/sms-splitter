import { SmsSplitter } from "../pkg/sms_splitter_wasm";

function test() {
  const message = "Hello World!";
  const splitter = new SmsSplitter();
  console.log(splitter.split(message));
}
test();