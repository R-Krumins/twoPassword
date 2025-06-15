<script lang="ts">
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import * as Card from "$lib/components/ui/card/index.js";
  import { KeyRound, ClipboardCopy, ChevronDown, Eye } from "@lucide/svelte";
  import { getVault } from "$lib/vault.js";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";

  async function copyToClipboard(text: string) {
    try {
      await writeText(text);
      console.log("Text copied to clipboard!");
    } catch (error) {
      console.error("Failed to copy text:", error);
    }
  }
  const vaultItems = getVault();
</script>

<div class="max-w-4xl mx-auto p-6 space-y-4">
  {#each vaultItems as item (item.name)}
    <Collapsible.Root>
      <Card.Root
        class="shadow-sm hover:shadow-md transition-shadow duration-200"
      >
        <Collapsible.Trigger
          class="p-6 font-semibold text-lg hover:bg-gray-50 transition-colors duration-150"
        >
          <div class="flex items-center gap-3">
            <KeyRound class="w-5 h-5 text-gray-600" />
            <span class="text-gray-900">{item.name}</span>
            <ChevronDown
              class="w-4 h-4 text-gray-400 ml-auto transition-transform duration-200 group-data-[state=open]:rotate-180"
            />
          </div>
        </Collapsible.Trigger>

        <Collapsible.Content class="border-t border-gray-100">
          <div class="p-6 space-y-6 bg-gray-50/50">
            <!-- Username Section -->
            <div class="space-y-2">
              <Label
                for="user-{item.name}"
                class="text-sm font-medium text-gray-700"
              >
                Username
              </Label>
              <div class="flex items-center gap-3">
                <Button
                  variant="outline"
                  size="sm"
                  class="px-3 py-2 hover:bg-gray-100 transition-colors"
                  aria-label="Copy username"
                  onclick={() => copyToClipboard(item.user)}
                >
                  <ClipboardCopy class="w-4 h-4" />
                </Button>
                <Input
                  id="user-{item.name}"
                  type="text"
                  value={item.user}
                  readonly
                  class="flex-1 bg-white border-gray-200"
                />
              </div>
            </div>

            <!-- Password Section -->
            <div class="space-y-2">
              <Label
                for="pswd-{item.name}"
                class="text-sm font-medium text-gray-700"
              >
                Password
              </Label>
              <div class="flex items-center gap-3">
                <Button
                  variant="outline"
                  size="sm"
                  class="px-3 py-2 hover:bg-gray-100 transition-colors"
                  aria-label="Copy password"
                  onclick={() => copyToClipboard(item.pswd)}
                >
                  <ClipboardCopy class="w-4 h-4" />
                </Button>
                <Input
                  id="pswd-{item.name}"
                  type="password"
                  value={item.pswd}
                  readonly
                  class="flex-1 bg-white border-gray-200"
                />
                <Button
                  variant="ghost"
                  size="sm"
                  class="px-3 py-2 hover:bg-gray-100 transition-colors"
                  aria-label="Toggle password visibility"
                >
                  <Eye class="w-4 h-4" />
                </Button>
              </div>
            </div>
          </div>
        </Collapsible.Content>
      </Card.Root>
    </Collapsible.Root>
  {/each}
</div>
