import { getData, SigningCosmWasmClient } from "./signer";
const { ADDR, CONTR, getAliceClient } = getData(true);

const l = console.log.bind(console);

async function main() {
  const aliceClient = (await getAliceClient(true)) as SigningCosmWasmClient;
  const gas = {
    amount: [{ denom: "ujunox", amount: "625" }],
    gas: "250000",
  };

  const query = async () => {
    let res = await aliceClient.queryContractSmart(CONTR.ADDR, {
      query_with_wasm_query: { contract_addr: CONTR.ADDR },
    });
    l("\n", res, "\n");
  };

  let res;

  await query();

  res = await aliceClient.execute(
    ADDR.ALICE,
    CONTR.ADDR,
    { set_with_msg: { contract_addr: CONTR.ADDR, count: 111 } },
    gas
  );
  l({ attributes: res.logs[0].events[2].attributes }, "\n");

  await query();

  res = await aliceClient.execute(
    ADDR.ALICE,
    CONTR.ADDR,
    { set_with_sub_msg: { contract_addr: CONTR.ADDR, count: 222 } },
    gas
  );
  l({ attributes: res.logs[0].events[2].attributes }, "\n");

  await query();
}

main();
