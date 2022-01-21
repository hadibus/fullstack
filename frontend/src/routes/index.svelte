<script>
	import Header from '$lib/header.svelte';
	import Footer from '$lib/footer.svelte';

	async function getMessage() {
		const res = await fetch('http://localhost:8080/hey');
		const text = await res.text();

		if (res.ok) {
			return text;
		} else {
			throw new Error(text);
		}
	}

	let message = (async () => 'message will be here')();

	function handleClick() {
		message = getMessage();
	}
</script>

<Header />

<main>
	<div>
		<h1>Welcome to Yesterworks!</h1>
		<h3>Our mission...</h3>
		<p>
			to provide you with knowledge and wisdom from the past and the
			materials you need in order to live a life close to the land. Our
			ancestors lived a such a life and lived it well. We believe that a
			life in harmony with the land and with less comforts and
			distractions of modernity is a life full of meaning and fulfillment.
			It is a life of less consumption and more creation. To stagnation we
			say, move over, it's time for something new. Join us, as this new
			life unfolds before you.
		</p>
		<button on:click={handleClick}> say hey </button>

		<p>latest blog posts</p>

		<p>hottest tools</p>

		{#await message}
			<p>loading...</p>
		{:then m}
			<p>server says: {m}</p>
		{:catch error}
			<p style="color: red">{error.message}</p>
		{/await}
	</div>
</main>

<Footer />
