 <script lang="ts">
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


<h1>Welcome to frontend</h1>

<button on:click={handleClick}>
	say hey
</button>

{#await message}
				<p>loading...</p>
{:then m}
				<p>server says: {m}</p>
{:catch error}
				<p style="color: red">{error.message}</p>
{/await}
