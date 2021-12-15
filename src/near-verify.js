import * as nearApi from "near-api-js";
import {sha256} from "js-sha256";

export function verify(text) {
    const account = window.walletConnection.account()
    const keyStore = account.connection.signer.keyStore;
    const privateKey=keyStore.localStorage[`${keyStore.prefix}${window.accountId}:${account.connection.networkId}`];
    const keyPair = new nearApi.utils.key_pair.KeyPairEd25519(privateKey.substring("ed25519:".length));
    const publicKey= keyPair.getPublicKey().toString();
    const message = new Uint8Array(sha256.array(text));
    const signature = keyPair.sign(message);
    const publicKey1 = nearApi.utils.key_pair.PublicKey.from(publicKey);
    return publicKey1.verify(message, signature.signature);
}
