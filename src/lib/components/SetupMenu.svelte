<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    let { onSuccess } = $props();

    let inputPassword = $state("");
    let inputPasswordConfirmation = $state("");
    let isSamePassword = $state(false)
    let errorMessage = $state("")

    async function checkSamePassword(event: Event) {
      event.preventDefault();
      if (inputPassword == "") {
        errorMessage = "You cannot have a blank master password!"
      }
      else if (inputPassword != inputPasswordConfirmation) { // inputedPassword = "" so the user cant input a blank password.
        isSamePassword = false;
        inputPasswordConfirmation = "";
        errorMessage = "The passwords does not matches!"
      } else {
        isSamePassword = true;
        try {
          await invoke("create_first_password", { masterPass: inputPassword });
          onSuccess();
          } catch (error) {
            console.error("Rust error :", error);
            errorMessage = "Rust error  : " + error;
          }
      }

      setTimeout(() => {
        errorMessage = "";
      }, 1500);

    }

</script>

<main>
    <form class="row" onsubmit={checkSamePassword}>
        <h1>Welcome on **find a name for the password manager**!</h1>
        <h2>You will have to choose a master password, it is really important for you to not forget it at all cost !!! </h2>
        <input type="password" id="inputPassword" bind:value={inputPassword} class="border p-2" placeholder="password"/> <br/>

        <input type="password" id="inputPasswordConfirmation" bind:value={inputPasswordConfirmation} class="border p-2" placeholder="confirmation password"/>

        <button type="submit" class="bg-blue-500 text-white p-2 mt-2 rounded">Register</button>

        <h3>{errorMessage}</h3>
    </form>
</main>
