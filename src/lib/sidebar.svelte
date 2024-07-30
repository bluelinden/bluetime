<script lang="ts">
	import { page } from '$app/stores';

	import homeIcon from '$icon/home.svg?raw';
	import healthIcon from '$icon/health_and_safety.svg?raw';
	import hourglassIcon from '$icon/hourglass.svg?raw';
	import settingsIcon from '$icon/settings.svg?raw';

	interface SidebarLink {
		href: string;
		title: string;
		iconRaw: string;
	}

	const links: SidebarLink[] = [
		{
			href: '/',
			title: 'Home',
			iconRaw: homeIcon
		},
		{
			href: '/limits',
			title: 'Limits',
			iconRaw: hourglassIcon
		},
		{
			href: '/wellbeing',
			title: 'Wellbeing',
			iconRaw: healthIcon
		}
	];

	const bottomLinks: SidebarLink[] = [
		{
			href: '/settings',
			title: 'Settings',
			iconRaw: settingsIcon
		}
	];

	let path: string;

	function getPath(currentPath: string) {
		path = currentPath;
	}

	$: getPath($page.url.pathname);
</script>

<nav
	id="mainnav"
	class="flex min-h-screen w-[180px] flex-col border-r-[1px] border-r-stone-900 bg-lime-950 py-3 text-lime-100"
>
	<h1 class="mb-8 text-center font-bold">Hourlasso</h1>
	<section class="mb-auto">
		{#each links as link}
			<a href={link.href} class:active={$page.url.pathname === link.href}
				>{@html link.iconRaw} {link.title}</a
			>
		{/each}
	</section >

	<section>
		{#each bottomLinks as link}
			<a href={link.href} class:active={$page.url.pathname === link.href}
				>{@html link.iconRaw} {link.title}</a
			>
		{/each}
	</section>
</nav>

<style>
	#mainnav a {
		@apply mx-2 mb-[2px] flex flex-row items-center rounded-md px-3 py-1.5 font-medium text-lime-100 transition-colors duration-75 hover:bg-lime-900;
		:global(svg) {
			@apply mb-[2px] mr-2 h-5 w-5 fill-lime-300;
		}
		&.active {
			@apply bg-lime-100 text-lime-900;
			:global(svg) {
				@apply fill-lime-900;
			}
		}
	}
</style>
