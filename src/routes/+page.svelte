<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from '@tauri-apps/plugin-opener';
  import { save, open } from "@tauri-apps/plugin-dialog";
  import MenuLogic from "$lib/components/LogicMenu.svelte";

  async function passwordExport() {
    try {
      const filePath = await save({
        title: 'Export your vault',
        defaultPath: 'passwords.json',
        filters: [{
          name: 'JSON',
          extensions: ['json']
          }]
        });

      await invoke("export_password", { destinationPath: filePath });
    } catch(err) {
      console.error("Couldn't export the passwords" + err);
    }
  }

  async function passwordImport() {
    try {
      const file = await open({
        multiple: false,
        directory: false,
      });

      await invoke("import_password", {  fromPath: file });

      window.location.reload(); // reload the page so we get back to the login page.
    }
    catch(err) {
      console.error("Couldn't import the password" + err);
    }
  }

  async function handleToolbarFile(event: Event) {
    const select = event.target as HTMLSelectElement; // <option value> itself
    const action: string = select.value; // Import or Export

    switch(action) {
      case "import":
        passwordImport();
        break;
      case "export":
        passwordExport();
        break;
    }

    select.value = "";
  }

  function handleToolbarHelp(event: Event) {
    const select = event.target as HTMLSelectElement;
    const action = select.value;

    if (action == "github") {
      openUrl("https://github.com/Anthoxoo/gui_password_manager.git");
    }

    select.value = "";

  }

</script>

<main class="container">
    <div class="toolbar">
        <select onchange={handleToolbarFile}>
            <option value="">File</option>
            <option value="import">Import</option>
            <option value="export">Export</option>

        </select>
        <select onchange={handleToolbarHelp}>
            <option value="">Help</option>
            <option value="github">Github</option>
        </select>
    </div>

    <h1>Hello and welcome on my future password manager!</h1>

    <MenuLogic />
</main>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

h1 {
  text-align: center;
}

.toolbar {
    text-align: left;
    display: fixed;
    top: 0;
    left: 0;

    color: #4b4f57;
}
@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

}

</style>
