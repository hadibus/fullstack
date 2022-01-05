 <script lang="ts">
  import Header from "./Header.svelte";
  import Footer from "./Footer.svelte";

	async function getMessage(){
		const res = await fetch('http://localhost:8080/hey')
		const text = await res.text();

		if (res.ok) {
			return text;
		} else {
			throw new Error(text);
		}
	}

  let message = (async () => 'message will be here')();

  function handleClick(){
		message = getMessage();
	}
</script>

<style>
	main {
    padding: 10px;
    flex: 1;
	}
</style>

<Header/>

<main>

  <h2>Welcome to Yesterworks!</h2>
  <button on:click={handleClick}>
	  say hey
  </button>

	<p>latest blog posts</p>

	<p>hottest tools</p>


  {#await message}
    <p>loading...</p>
  {:then m}
    <p>server says: {m}</p>
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
</main>

<Footer/>
