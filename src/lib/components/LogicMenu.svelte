<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { StateMenu } from "../typesMenu";
    import LoadingMenu from "./LoadingMenu.svelte";
    import SetupMenu from "./SetupMenu.svelte";
    import LoginMenu from "./LoginMenu.svelte";
    import VaultMenu from "./VaultMenu.svelte";

    let currentStateMenu = $state(StateMenu.Loading)
    onMount(async () => {
      try {
        let isFirstRegister = await invoke("is_first_launch");

        if (isFirstRegister) {
          currentStateMenu = StateMenu.Setup
          console.log("Setup")
        } else {
          currentStateMenu = StateMenu.Login
          console.log("Login")
        }
      } catch(err) {
        console.error("Error connecting to rust:", err)
      }
    });

</script>
<div class="flex flex-col items-center">
    {#if currentStateMenu === StateMenu.Loading}
        <LoadingMenu />

    {:else if currentStateMenu === StateMenu.Setup}
        <SetupMenu onSuccess={() => currentStateMenu = StateMenu.Login} />

    {:else if currentStateMenu === StateMenu.Login}
        <LoginMenu />

    {:else if currentStateMenu === StateMenu.Vault}
        <VaultMenu />
    {/if}
</div>
