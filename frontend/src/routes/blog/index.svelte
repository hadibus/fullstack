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
</script>

<Header />

<main>
	<div>
		<h2>Blog Posts</h2>
		<ul>
			{#each posts as post}
				<li>
					<h2>
						<a href={post.path}>
							{post.meta.title}
						</a>
					</h2>
					Published {post.meta.date}
				</li>
			{/each}
		</ul>
	</div>
</main>

<Footer />

<style>
	main {
		padding: 10px;
		flex: 1;
		justify-content: center;
		display: flex;
	}

	main > div {
		max-width: 1120px;
		flex-grow: 100;
	}
</style>
