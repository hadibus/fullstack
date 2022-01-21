<script>
  import Header from "$lib/header.svelte";
  import Footer from "$lib/footer.svelte";

  export let title;
  export let author;
  export let date;
</script>

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

  .title {
    font-size: 40px;
  }
</style>

<Header/>

<main>
        <div>
<h1 class="title">{ title }</h1>
<p class="date">on: { date }</p>
<p class="date">by: { author }</p>
<slot>
  <!-- the mdsvex content will be slotted in here -->
</slot>
        </div>
</main>

<Footer/>
