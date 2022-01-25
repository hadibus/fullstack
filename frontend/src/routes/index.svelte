<script context="module">
	export const load = async ({ fetch }) => {
		const posts = await fetch('/blog/posts.json');
		const allPosts = await posts.json();

		return {
			props: {
				posts: allPosts
			}
		};
	};
</script>

<script>
	import Header from '$lib/header.svelte';
	import Footer from '$lib/footer.svelte';

	export let posts;

	async function getMessage() {
		const res = await fetch('/api/hey');
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
	<div class="upper-image">
		<div>
			<h1>Welcome</h1>
			<h1>to</h1>
			<h1>Yesterworks!</h1>
		</div>
	</div>
	<div>
		<div>
			<div class="note-container">
				<div class="note">
					<p>This is the home page. Features include:</p>
					<ul>
						<li>Responsive design</li>
						<li>Pleasing large image</li>
						<li>Mission statement</li>
						<li>Button to send API call to back end</li>
						<li>Sticky footer</li>
					</ul>
				</div>
			</div>
			<h2>Our mission...</h2>
			<p>
				to provide you with knowledge and wisdom from the past coupled
				with the new knowlege of today to help you create the world
				around youself and your family.
			</p>
			<p>
				We favor creating things with raw materials to using processed
				materials. We believe that self-reliance is not simply a
				financial choice, but also an expression of freedom. We believe
				that a life with less comforts and distractions of modernity is
				a life full of meaning and fulfillment. It is a life of less
				consumption and more creation. To stagnation we say, move over,
				it's time for something new. Join us, as this new life unfolds
				before you.
			</p>
		</div>
	</div>

	<div class="dark">
		<div>
			<h2>Send request to back end</h2>
			<p> Here you can send an http request to the back-end API.</p>

		<button on:click={handleClick}> say hey to the back end</button>

		{#await message}
			<p>loading...</p>
		{:then m}
			<p>server says: {m}</p>
		{:catch error}
			<p style="color: red">{error.message}</p>
		{/await}
		</div>
	</div>
	<div>
		<div>
			<h2>latest blog posts</h2>
			<ul class="blog-posts">
				{#each posts as post}
					<li class="blog-post">
						<h2>
							<a href={post.path}>
								{post.meta.title}
							</a>
						</h2>
						<p>Published {post.meta.date}</p>
					</li>
				{/each}
			</ul>

			<h2>hottest tools</h2>
			<p>Not yet implemented</p>
		</div>
	</div>
</main>

<Footer />

<style>
	.upper-image {
		background-image: url('/woodworking_0.jpg');
		width: 100%;
		padding-top: 150px;
		padding-bottom: 150px;
		background-position: top center;
	}

	.upper-image > div {
		justify-content: center;
		display: flex;
		flex-wrap: wrap;
	}

	.upper-image > div > * {
		color: white;
		padding-left: 4px;
		padding-right: 4px;
		font-size: 40px;
	}

	main > div > * {
		padding: 20px;
	}

	.note-container {
		display: flex;
		justify-content: left;
	}

	.note {
		border-radius: 25px;
		border: 2px solid green;
		background-color: lightgreen;
		padding: 20px;
		padding-top: 10px;
	}

	.blog-posts {
		max-width: 700px;
		min-width: 300px;
	}

	.blog-post {
		flex-wrap: wrap;
		border: 2px solid black;
		display: flex;
		justify-content: space-between;
		padding-left: 10px;
		padding-right: 10px;
		margin: 10px;
	}

	.dark {
		background-color: tan;
		padding-top: 150px;
		padding-bottom: 150px;
	}
</style>
