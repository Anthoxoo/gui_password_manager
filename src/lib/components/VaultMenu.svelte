<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let { onDisconnect } = $props()
    let errorMessage = $state("")
    let passwordsList: [String, String, String][] = $state([]);
    onMount(async () => {
      try {
        passwordsList = await invoke("password_to_vec");
      } catch(err) {
        console.error("error listing passwords", err)
        errorMessage = "Error listing passwords : " + err
      }
    })
</script>

<main>
    <h2>Welcome back in your vault !</h2>
    {#each passwordsList as [url, username, password]}
        <p><strong>url: </strong> {url} | <strong> username: </strong> {username} | <strong> password: </strong> {password} </p>
    {/each}

    {#if passwordsList.length === 0}
        <p class="text-gray-500">Your vault is empty!.</p>
    {/if}
    <p>{errorMessage}</p>
    <form class="row" onsubmit={onDisconnect}>
        <button type="submit">Lock your vault</button>
    </form>
    <!-- <form class="row" onsubmit={}></form> -->

</main>
