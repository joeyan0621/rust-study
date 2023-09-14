import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import "@polkadot/api-augment";
import type { KeyringPair } from "@polkadot/keyring/types";

import { WS_SOCKET } from "./utils/constants";
type SomethingStoredType = {
  who: string;
  something: string;
};
async function connect() {
  const provider = new WsProvider(WS_SOCKET);
  const api = await ApiPromise.create({ provider, types: {} });
  await api.isReady;
  return api;
}

export async function init() {
  const api = await connect();
  (document.getElementById("button") as HTMLButtonElement).addEventListener(
    "click",
    () => {
      changeSomethingValue(api);
    }
  );
  await subscribeSomething(api);
  await subscribeEvent(api);
}
function changeSomethingValue(api: ApiPromise) {
  const keyring = new Keyring({
    type: "sr25519",
  });
  const account = keyring.addFromUri("//Alice");
  const input = document.getElementById("number") as HTMLInputElement;
  const inputValue = input.value || "";
  if (!inputValue.trim()) {
    alert("请输入数字");
  } else {
    doSomething(api, account, inputValue);
  }
}

async function doSomething(
  api: ApiPromise,
  account: KeyringPair,
  value: string
) {
  const buttonEle = document.getElementById("button") as HTMLButtonElement;
  buttonEle.innerText = "正在修改something...";
  buttonEle.disabled = true;
  api.tx.templateModule.doSomething(value).signAndSend(account, (res) => {
    console.log(res.isCompleted);
    if (res.isCompleted) {
      buttonEle.innerText = "修改something的值";
      buttonEle.disabled = false;
    }
  });
}

async function subscribeEvent(api: ApiPromise) {
  const PalletName = "templateModule";
  await api.query.system.events((events) => {
    events.forEach((item) => {
      const { event } = item;
      if (event.section === PalletName) {
        // const { who } = event.data.toHuman() as SomethingStoredType;
        // const info = `${who}触发事件： ${event.method}`;
        const info = ` ${event.method}事件触发`;
        const infoElement = document.createElement("div");
        infoElement.innerText = info;
        const infoContainer = document.getElementById("info");
        infoContainer?.appendChild(infoElement);
      }
    });
  });
}

async function subscribeSomething(api: ApiPromise) {
  api.query.templateModule.something((res) => {
    console.log("🐼🐼🐼🐼 something的值为：", res.value.toHuman());
    const value = res.value.toHuman() as string;
    const valueElement = document.getElementById(
      "something"
    ) as HTMLSpanElement;
    valueElement.innerText = value;
  });
}

init();
