import { writable } from "svelte/store";
import { get } from "svelte/store";

type VaultItem = {
  name: string;
  user: string;
  pswd: string;
};

export const vault = writable<VaultItem[]>([]);

export function setVault(data: string) {
  try {
    const parsedData = JSON.parse(data);
    if (Array.isArray(parsedData)) {
      vault.set(parsedData);
    } else {
      console.error("Parsed data is not an array:", parsedData);
    }
  } catch (error) {
    console.error("Error parsing vault data:", error);
  }
}

export function getVault(): VaultItem[] {
  return get(vault);
}
