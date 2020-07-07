<script>
	import * as did from 'iota-did-wasm/wasm-web'
	console.log("did", did)

	export let name;
	export let seed;
	try {
		seed = did.GenerateSeed();
	} catch (error) {
		seed = error
	}
	console.log("seed", seed)

</script>

<style>
	h1 {
		color: purple;
	}
</style>

<main>
	<h1>Hello {name}!</h1>
	<p>Seed: {seed}</p>
</main>