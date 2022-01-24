<script>
	import Header from '$lib/header.svelte';
	import Footer from '$lib/footer.svelte';

	export let title;
	export let author;
	export let date;
</script>

<Header />

<main>
	<div>
		<div>
			<h1 class="title">{title}</h1>
			<p class="date">on: {date}</p>
			<p class="date">by: {author}</p>
			<slot>
				<!-- the mdsvex content will be slotted in here -->
			</slot>
		</div>
	</div>
</main>

<Footer />
