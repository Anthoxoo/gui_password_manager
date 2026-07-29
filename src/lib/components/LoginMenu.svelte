<script lang="ts">

    let inputPassword = $state("");
    let seePassword = $state(false);
    function tooglePassword() {
      seePassword = !seePassword;
    }

</script>

<main>
    <h1>This is the login menu</h1>

    <form class="row">
        <h2>Enter your master password to unlock the vault : </h2>
        {#if seePassword === false}
            <input type="password" id="inputPassword" bind:value={inputPassword} class="border p-2" />
        {:else}
            <input type="text" id="inputPassword" bind:value={inputPassword} class="border p-2" />
        {/if}
        <button type="submit" class="bg-blue-500 text-white p-2 mt-2 rounded">Unlock</button>
        <input type="checkbox" onclick={tooglePassword}>Show Password
    </form>
</main>
