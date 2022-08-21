import { getData, SigningCosmWasmClient, fromBinary } from "./signer";
const { ADDR, CONTR, getAliceClient } = getData(true);

const CNT_CONTR_ADDR =
  "juno1f4j9ngmnz6r803x5p3xg8j450y6clc4vydsyxegtpz5kfvffeyeq4xh448";

const l = console.log.bind(console);

async function main() {
  const aliceClient = (await getAliceClient(true)) as SigningCosmWasmClient;
  const gas = {
    amount: [{ denom: "ujunox", amount: "625" }],
    gas: "250000",
  };

  const query = async () => {
    let res = await aliceClient.queryContractSmart(CONTR.ADDR, {
      smart_query: { contract_addr: CNT_CONTR_ADDR },
    });
    // let {
    //   data: {
    //     wasm: {
    //       execute: { msg },
    //     },
    //   },
    // } = res;
    // msg = fromBinary(msg);
    l("\n", res, "\n"); // { get_count: {}
  };

  await query();

  let res;

  // res = await aliceClient.execute(
  //   ADDR.ALICE,
  //   CONTR.ADDR,
  //   {
  //     set_with_msg: {
  //       contract_addr: CNT_CONTR_ADDR,
  //       count: 111,
  //     },
  //   },
  //   gas
  // );
  // l({ attributes: res.logs[0].events[2].attributes }, "\n");

  // {
  //   attributes: [
  //     {
  //       key: "_contract_address",
  //       value:
  //         "juno104lnwqak6t37nn4llr4nwdc365zlskhrc2dfsgwgu6nyfa007yzquvy4wh",
  //     },
  //     { key: "method", value: "set_with_msg" },
  //     { key: "expected_count", value: "111" },
  //     {
  //       key: "_contract_address",
  //       value:
  //         "juno1f4j9ngmnz6r803x5p3xg8j450y6clc4vydsyxegtpz5kfvffeyeq4xh448",
  //     },
  //     { key: "method", value: "set" },
  //     {
  //       key: "owner",
  //       value: "juno1gjqnuhv52pd2a7ets2vhw9w9qa9knyhyqd4qeg",
  //     },
  //     { key: "count", value: "111" },
  //   ];
  // }

  // res = await aliceClient.execute(
  //   ADDR.ALICE,
  //   CONTR.ADDR,
  //   { set_with_sub_msg: { contract_addr: CNT_CONTR_ADDR, count: 222, id: 1 } },
  //   gas
  // );
  // l({ attributes: res.logs[0].events[2].attributes }, "\n");

  /*
  Error: Error when broadcasting tx C0F2A7801A776BA2A74E1C2EBF3C3DB7D45CDC082CE80BEFF24A6C3778CE09E7
  at height 1332826. Code: 5; Raw log: failed to execute message; message index: 0:
  dispatch: submessages: reply: Error calling the VM: Error resolving Wasm function:
  Could not get export: Missing export reply: execute wasm contract failed
  at SigningCosmWasmClient.execute (/home/fewed/local-cnt-trigger/scripts/node_modules/@cosmjs/cosmwasm-stargate/src/signingcosmwasmclient.ts:410:13)
  */
}

main();
