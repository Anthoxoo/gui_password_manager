<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    let { onSuccess } = $props()

    let inputPassword = $state("");
    let seePassword = $state(false);
    function tooglePassword() {
      seePassword = !seePassword;
    }

    let errorMessage = $state("");
    async function passwordCheck(event: Event) {
      event.preventDefault();

      try {
        let isPasswordCorrect = await invoke("check_password", { masterPass: inputPassword });

        if (isPasswordCorrect) {
        onSuccess();
        } else {
          errorMessage = "Incorrect password!"
        }
      } catch(err) {
        errorMessage = "Error checking passwords : " + err;
      }

      setTimeout(() => {
        errorMessage = "";
      }, 1500);

    }


</script>

<main>
    <h1>This is the login menu</h1>

    <form class="row" onsubmit={passwordCheck}>
        <h2>Enter your master password to unlock the vault : </h2>
        {#if seePassword === false}
            <input type="password" id="inputPassword" bind:value={inputPassword} class="border p-2" />
        {:else}
            <input type="text" id="inputPassword" bind:value={inputPassword} class="border p-2" />
        {/if}
        <button type="submit" class="bg-blue-500 text-white p-2 mt-2 rounded">Unlock</button>
        <input type="checkbox" onclick={tooglePassword}>Show Password
        {#if errorMessage !== ""}
            <h3 class="text-red-500 mt-2">{errorMessage}</h3>
        {/if}
    </form>
</main>
