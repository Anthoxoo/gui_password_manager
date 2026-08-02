<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let { onDisconnect } = $props()

    let errorMessage = $state("");
    let passwordsList: [string, string, string][] = $state([]);
    async function fetchPasswords() {
      try {
          passwordsList = await invoke("password_to_vec");
      } catch(err) {
        console.error("error listing passwords", err);
        errorMessage = "Error listing passwords : " + err;
      }
    }

    onMount(async () => {
      await fetchPasswords();
    })


    let showAddPassword = $state(false);
    function toggleAddPassword(event: Event) {
      event.preventDefault();
      errorMessage = "";
      showAddPassword = !showAddPassword;
    }

    let addUrlInput = $state("");
    let addUsernameInput = $state("");
    let addPasswordInput = $state("");

    async function addPassword(event: Event) {
      event.preventDefault();

      try {
        await invoke("add_password", { url: addUrlInput, username: addUsernameInput, password: addPasswordInput })
        await fetchPasswords();

        addUrlInput = "";
        addUsernameInput = "";
        addPasswordInput = "";
        showAddPassword = false;

      } catch(err) {
        console.error("Error adding new entry: ", err);
        errorMessage = "Error creating new entry : " + err
      }
    }

    let removeEntryUrl = $state("");
    async function deleteEntry(event: Event) {
      event.preventDefault();

      try {
        await invoke("delete_entry", { url: removeEntryUrl })
        await fetchPasswords();
      } catch(err) {
        console.error(err)
        errorMessage = "Error: " + err
      }
    }

    let showRemoveEntry = $state(false);
    function toggleRemoveEntry(event: Event) {
      event.preventDefault();
      errorMessage = "";
      showRemoveEntry = !showRemoveEntry;
    }

</script>

<main>
    <h2>Welcome back in your vault !</h2>
    {#each passwordsList as [url, username, password]}
        <p><strong>url: </strong> {url} | <strong> username: </strong> {username} | <strong> password: </strong> {password} </p>
    {/each}

    {#if passwordsList.length === 0}
        <p class="text-gray-500">Your vault is empty!</p>
    {/if}
    <p>{errorMessage}</p>

    <form class="row" onsubmit={toggleAddPassword}>
        <button type="submit">Add an entry</button>
    </form>

    {#if showAddPassword}
        <form class="row" onsubmit={addPassword}>
            <input type="text" bind:value={addUrlInput} class="border p-2" placeholder="url"/>
            <input type="text" bind:value={addUsernameInput} class="border p-2" placeholder="username"/>
            <input type="password" bind:value={addPasswordInput} class="border p-2" placeholder="password"/>

            <button type="submit">add</button>
        </form>
    {/if}

    <form class="row" onsubmit={toggleRemoveEntry}>
        <button type="submit">Remove an entry</button>
    </form>

    {#if showRemoveEntry}
        <form class="row" onsubmit={deleteEntry}>
            <input type="text" bind:value={removeEntryUrl} class="border p-2" placeholder="url" />
        </form>
    {/if}

    <form class="row" onsubmit={onDisconnect}>
        <button type="submit">Lock your vault</button>
    </form>

</main>
