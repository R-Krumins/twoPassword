<script lang="ts">
  import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { on } from "svelte/events";

  type Props = {
    onAddItem: (name: string, user: string, pswd: string) => void;
  };

  const { onAddItem }: Props = $props();

  function handleSubmit(event: Event) {
    event.preventDefault();
    onAddItem(name, user, pswd);
    name = "";
    user = "";
    pswd = "";
  }

  let name = "";
  let user = "";
  let pswd = "";
</script>

<Dialog.Root>
  <Dialog.Trigger class={buttonVariants({ variant: "default" })}
    >Add to Vault</Dialog.Trigger
  >
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Add new Vault item</Dialog.Title>
    </Dialog.Header>
    <form class="grid gap-4 py-4" onsubmit={(e) => handleSubmit(e)}>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="name" class="text-right">Name</Label>
        <Input id="name" bind:value={name} class="col-span-3" required />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="user" class="text-right">Username</Label>
        <Input id="user" bind:value={user} class="col-span-3" required />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="pswd" class="text-right">Password</Label>
        <Input
          id="pswd"
          type="password"
          bind:value={pswd}
          class="col-span-3"
          required
        />
      </div>
      <Dialog.Footer>
        <Dialog.Close>
          <Button type="submit">Add</Button>
        </Dialog.Close>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
