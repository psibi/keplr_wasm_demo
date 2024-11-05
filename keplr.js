export async function enable_keplr(chain_ids) {
    await window.keplr.enable(chain_ids)
}

export async function get_accounts(chain_id) {
    const offlineSigner = window.keplr.getOfflineSigner(chain_id);
    const accounts = await offlineSigner.getAccounts();
    console.log(accounts);
    return accounts
}
