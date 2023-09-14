import { ApiPromise, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

async function setAccounts(api: ApiPromise) {
  const accounts = await api.query.system.account.entries();
  var select = document.getElementById("account-list")! as HTMLSelectElement;
  var fragment = document.createDocumentFragment();
  accounts.forEach(([address]) => {
    var option = document.createElement("option");
    const addressStr = address.args[0].toString();
    option.text = addressStr;
    option.value = addressStr;
    fragment.appendChild(option);
  });
  select.appendChild(fragment);
}

export { setAccounts };
