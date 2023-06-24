<script lang="ts">
	import { onMount } from 'svelte';

	let data: String;

	const example = async () => {
		let url = import.meta.env.VITE_BACKEND_URL + 'example/';
		await fetch(url, {
			method: 'GET',
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include'
		})
			.then((res) => {
				return res.json();
			})
			.then((body) => {
				data = body;
			});
	};

	onMount(() => {
		example();
	});
</script>

{#if data}
	<h1>{data}</h1>
{/if}
