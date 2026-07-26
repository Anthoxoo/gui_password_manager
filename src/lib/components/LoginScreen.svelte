<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    function clearPassword() {
      inputPassword = "";
    }
    let isConnected = $state(false)
    function toggleConnected(event: Event) {
      event.preventDefault()
      isConnected = !isConnected;

      clearPassword();
    }

    let inputPassword = $state("");
    let seePassword = $state(false);
    function tooglePassword() {
      seePassword = !seePassword;
    }
</script>

<div class="flex flex-col items-center">
    {#if !isConnected}
        <form class="row" onsubmit="{toggleConnected}"> <!-- check if the password is correct before toggle connection state -->
            <h2>Enter your master password to unlock the vault : </h2>
            {#if seePassword === false}
                <input type="password" id="inputPassword" bind:value={inputPassword} class="border p-2" />
            {:else}
                <input type="text" id="inputPassword" bind:value={inputPassword} class="border p-2" />
            {/if}
            <button type="submit" class="bg-blue-500 text-white p-2 mt-2 rounded">Unlock</button>
            <input type="checkbox" onclick={tooglePassword}>Show Password
        </form>
    {:else}
        <h2>Welcome back in your vault !</h2>
        <p>Shows menu</p>
        <form onsubmit={toggleConnected}>
            <button type="submit">Lock your vault</button>
        </form>
    {/if}
</div>
