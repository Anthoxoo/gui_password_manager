<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { StateMenu } from "../typesMenu";
    import LoadingMenu from "./LoadingMenu.svelte";
    import SetupMenu from "./SetupMenu.svelte";
    import LoginMenu from "./LoginMenu.svelte";
    import VaultMenu from "./VaultMenu.svelte";

    const viewMap = {
            [StateMenu.Loading]: LoadingMenu,
            [StateMenu.Setup]: SetupMenu,
            [StateMenu.Login]: LoginMenu,
            [StateMenu.Vault]: VaultMenu
        };

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
    let ActiveComponent = $derived(viewMap[currentStateMenu]);



    // let isConnected = $state(false)
    // function toggleConnected(event: Event) {
    //   event.preventDefault()
    //   isConnected = !isConnected;


</script>
<div class="flex flex-col items-center">
    <ActiveComponent />
</div>
