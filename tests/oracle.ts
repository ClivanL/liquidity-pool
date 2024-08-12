import * as sb from "@switchboard-xyz/on-demand";
import { PublicKey, Keypair } from "@solana/web3.js";

export async function loadDataFeed(){
    const ORACLE_FEED_ADDRESS = new PublicKey("q7AQr3jWfuKjeSy5anC4jz79gkgKrHnCNGd2xxTi9Mo");
    const { keypair, connection, program} = await sb.AnchorUtils.loadEnv();
    let feedAccount = new sb.PullFeed(program, ORACLE_FEED_ADDRESS);
    const [pullIx, responses, _ok, luts] = await feedAccount.fetchUpdateIx();
    return pullIx;
}
