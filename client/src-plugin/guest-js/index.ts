import { invoke } from '@tauri-apps/api/core';

export async function checkFileIntegrity(
  file: ArrayBuffer,
  sig: ArrayBuffer
) {
  const file_raw = new Uint8Array(
    file
  );

  const sig_raw = new Uint8Array(
    sig
  );

  return await invoke<boolean>("plugin:ahqai|check_file_integrity", {
    file: file_raw,
    sig: sig_raw
  })
}

export async function checkServerIntegrity(
  data: ArrayBuffer,
  sig: ArrayBuffer,
  pubkey: Uint8Array,
) {
  const data_raw = new Uint8Array(data);

  const sig_raw = new Uint8Array(sig);

  return await invoke<boolean>("plugin:ahqai|check_resp_integrity", {
    resp: data_raw,
    sig: sig_raw,
    pubkey: pubkey
  })
}

