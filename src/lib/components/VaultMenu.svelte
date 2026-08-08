<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let { onDisconnect } = $props()

    let filterValue = $state("");

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

    let filteredPasswordList = $derived(passwordsList.filter(([url, usernae, password]) => {
      return url.toLowerCase().includes(filterValue.toLowerCase());
    }));

    onMount(async () => {
      await fetchPasswords();
    })


    let showAddPassword = $state(false);
    function toggleAddPassword(event: Event) {
      event.preventDefault();
      errorMessage = "";
      showAddPassword = !showAddPassword;
    }

    let showHiddenAddPassword = $state(false);
    let showHiddenModifyPassword = $state(false);
    function toggleShowAddPassword(event: Event) {
      event.preventDefault();
      showHiddenAddPassword = ! showHiddenAddPassword;
    }
    function toggleShowModifyPassword(event: Event) {
      event.preventDefault();
      showHiddenAddPassword = ! showHiddenAddPassword;
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

    let removeEntryUrlInput = $state("");
    async function deleteEntry(event: Event) {
      event.preventDefault();

      try {
        await invoke("delete_entry", { url: removeEntryUrlInput })
        await fetchPasswords();

        removeEntryUrlInput = "";
        showRemoveEntry = false;

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

    let modifyUrlInput = $state("");
    let modifyUsernameInput = $state("");
    let modifyPassowrdInput = $state("");

    async function modifyEntry(event: Event) {
      event.preventDefault();

      try {
        await invoke("modify_entry", { url: modifyUrlInput, username: modifyUsernameInput, newPassword: modifyPassowrdInput })
        await fetchPasswords();

        modifyPassowrdInput = "";
        modifyUsernameInput = "";
        modifyUrlInput = "";
        showModifyEntry = false;

      } catch(err) {
        console.error(err);
        errorMessage = "Error: " + err;
      }
    }

    let showModifyEntry = $state(false)
    function toggleModifyEntry(event: Event) {
      event.preventDefault();
      errorMessage = "";
      showModifyEntry = !showModifyEntry;
    }

    let copyMessage = $state("");
    async function copyToClipboard(textToCopy: string) {
      try {
        await navigator.clipboard.writeText(textToCopy);

        copyMessage = "Field copied.";

        setTimeout(() => {
          copyMessage = "";
        }, 1500);
      } catch(err) {
        console.error("Error copying : " + err);
      }
    }
</script>

<main>
    <h2>Welcome back in your vault !</h2>
    <input type="search" placeholder="Search an url" bind:value={filterValue} />

    {#each filteredPasswordList as [url, username, password]}
        <br/>
        <strong>url: </strong> {url} |
        <strong> username: </strong> <span class="password-clickable" onclick={() => copyToClipboard(username)}> {username} </span> |
        <strong>Password: </strong> <span class="password-clickable" onclick={() => copyToClipboard(password)}> {password} </span>
    {/each}

    {#if passwordsList.length === 0}
        <p class="text-gray-500">Your vault is empty!</p>
    {/if}
    <p>{copyMessage}</p>
    <p>{errorMessage}</p>

    {#if filteredPasswordList.length === 0 && passwordsList.length != 0}
        <p>Nothing like "{filterValue}" is present in the vault</p>
    {/if}

    <form class="row" onsubmit={toggleAddPassword}>
        <button type="submit">Add an entry</button>
    </form>

    {#if showAddPassword}
        <form class="row" onsubmit={addPassword}>
            <input type="text" bind:value={addUrlInput} class="border p-2" placeholder="url"/>
            <input type="text" bind:value={addUsernameInput} class="border p-2" placeholder="username"/>
            {#if showHiddenAddPassword === false}
                <input type="password" bind:value={addPasswordInput} class="border p-2" placeholder="password" />
            {:else}
                <input type="text" bind:value={addPasswordInput} class="border p-2" placeholder="password" />
            {/if}
            <input type="checkbox" onclick={toggleShowAddPassword}>Show Password

            <br/>
            <button type="submit">add</button>
        </form>
    {/if}

    <form class="row" onsubmit={toggleRemoveEntry}>
        <button type="submit">Remove an entry</button>
    </form>

    {#if showRemoveEntry}
        <form class="row" onsubmit={deleteEntry}>
            <input type="text" bind:value={removeEntryUrlInput} class="border p-2" placeholder="url" />
        </form>
    {/if}

    <form class="row" onsubmit={toggleModifyEntry}>
        <button type="submit">Modify an entry</button>
    </form>

    {#if showModifyEntry}
        <form class="row" onsubmit={modifyEntry}>
            <input type="text" bind:value={modifyUrlInput} class="border p-2" placeholder="url" />
            <br/>
            <input type="text" bind:value={modifyUsernameInput} class="border p-2" placeholder="username" />
            {#if showHiddenModifyPassword === false}
                <input type="password" bind:value={modifyPassowrdInput} class="border p-2" placeholder="password" />
            {:else}
                <input type="text" bind:value={modifyPassowrdInput} class="border p-2" placeholder="password" />
            {/if}
            <button type="submit">modify</button>

            <br/>
            <input type="checkbox" onclick={toggleShowModifyPassword}>Show Password
        </form>
    {/if}


    <form class="row" onsubmit={onDisconnect}>
        <button type="submit">Lock your vault</button>
    </form>

</main>

<style>
    .password-clickable:hover {
        text-decoration: underline;
    }
</style>
