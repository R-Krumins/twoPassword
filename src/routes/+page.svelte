<script lang="ts">
  import { Button } from "$lib/components/ui/button/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Card from "$lib/components/ui/card/index.js";

  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import { setVault } from "$lib/vault.js";

  let path: string = "";
  let key: string = "";
  let errorMsg: string | null = null;

  function handleUnlockVault(command: string) {
    invoke(command, {
      path,
      key,
    })
      .then((res) => {
        console.log(command, "Successs!");
        setVault(res as string);
        goto("/vault");
      })
      .catch((error) => {
        console.log(command, "Fail!");
        errorMsg = error;
      });
  }
</script>

<div class="flex items-center justify-center min-h-screen">
  <Card.Root class="w-full max-w-sm">
    <Card.Header>
      <Card.Title>Unlock your vault</Card.Title>
    </Card.Header>

    <Card.Content>
      <form>
        <div class="flex flex-col gap-6">
          <div class="grid gap-2">
            <Label for="vault-path">Vault Path</Label>
            <Input
              id="vault-path"
              type="text"
              placeholder="/path/to/vault"
              required
              bind:value={path}
            />
          </div>

          <div class="grid gap-2">
            <Label for="vault-key">Password</Label>
            <Input id="vault-key" type="password" required bind:value={key} />
          </div>
        </div>
      </form>

      {#if errorMsg}
        <div class="text-red-500 mt-2">
          <p>{errorMsg}</p>
        </div>
      {/if}
    </Card.Content>

    <Card.Footer class="flex-col gap-2">
      <Button
        type="submit"
        class="w-full"
        onclick={() => handleUnlockVault("open_vault")}>Unlock</Button
      >
      <Button
        variant="outline"
        class="w-full"
        onclick={() => handleUnlockVault("create_new_vault")}>New Vault</Button
      >
    </Card.Footer>
  </Card.Root>
</div>
