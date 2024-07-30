<script lang="ts">
	import { ModeWatcher } from 'mode-watcher';
	import '../app.css';
	import Sidebar from '$lib/sidebar.svelte';
	import { onMount } from 'svelte';

	onMount(() => {
		setupAppWindow();
		setTimeout(() => {
			let splash = document.getElementById('splash');
			let main = document.getElementById('main');

			if (splash) {
				splash.style.display = 'none';
			}
			if (main) {
				main.style.display = 'contents';
			}
		}, 1000);
	});

	async function setupAppWindow() {
		const appWindow = (await import('@tauri-apps/api/window')).appWindow;
		appWindow.show();
	}
</script>

<ModeWatcher />
<div class="flex w-screen flex-row">
	<div>
		<Sidebar />
	</div>
	<main class="flex min-h-screen w-full flex-col bg-lime-100 text-lime-800">
		<slot></slot>
	</main>
</div>
