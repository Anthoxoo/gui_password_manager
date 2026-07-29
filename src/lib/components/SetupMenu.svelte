<script lang="ts">

    let inputPassword = $state("");
    let inputPasswordConfirmation = $state("");
    let isSamePassword = $state(false)
    let errorMessage = $state("")

    function checkSamePassword(event: Event) {
      event.preventDefault()

      if (inputPassword != inputPasswordConfirmation || inputPassword == "") { // inputedPassword = "" so the user cant input a blank password.
        isSamePassword = false;
        inputPasswordConfirmation = "";
        errorMessage = "The passwords does not matches!"
        //call new fn from rust here
      } else { isSamePassword = true }
    }

</script>

<main>
    <form class="row" onsubmit={checkSamePassword}>
        <h1>Welcome on **find a name for the password manager**!</h1>
        <h2>You will have to choose a master password, it is really important for you to not forget it at all cost !!! </h2>
        <input type="password" id="inputPassword" bind:value={inputPassword} class="border p-2" placeholder="password"/> <br/>

        <input type="password" id="inputPasswordConfirmation" bind:value={inputPasswordConfirmation} class="border p-2" placeholder="confirmation password"/>

        <button type="submit" class="bg-blue-500 text-white p-2 mt-2 rounded">Register</button>

        {#if isSamePassword}
            <h3>call new function from backend and go on login page</h3>

        {:else if errorMessage != ""}
            <h3>{errorMessage}</h3>
        {/if}
    </form>
</main>
