<script>
	let isMenuOpen = false;

	function toggleMenu() {
		isMenuOpen = !isMenuOpen;
	}

	function handleClickOutside(event) {
		const mobileMenu = document.getElementById('mobile-menu');
		const hamburgerButton = event.target.closest('button');

		if (!hamburgerButton && !mobileMenu?.contains(event.target)) {
			isMenuOpen = false;
		}
	}
</script>

<svelte:window on:click={handleClickOutside} />

<header
	class="fixed left-0 right-0 top-0 z-50 flex w-full items-center justify-end bg-neutral-800/90 px-4 py-3 text-white backdrop-blur-sm transition-all duration-300 ease-in-out md:px-16"
>
	<button
		class="ml-auto block rounded p-2 hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-blue-500 md:hidden"
		on:click={toggleMenu}
		aria-labelledby="menu-button-text"
	>
		<svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="2"
				d="M4 6h16M4 12h16M4 18h16"
			></path>
		</svg>
	</button>

	<nav class="hidden w-full justify-end md:flex md:items-center md:gap-8">
		<a
			href="#home"
			class="relative font-medium transition-colors duration-200 after:absolute after:bottom-[-4px] after:left-0 after:h-[2px] after:w-0 after:bg-blue-400 after:transition-all after:duration-300 hover:text-blue-400 hover:after:w-full"
		>
			Home
		</a>
		<a
			href="#experience"
			class="relative font-medium transition-colors duration-200 after:absolute after:bottom-[-4px] after:left-0 after:h-[2px] after:w-0 after:bg-blue-400 after:transition-all after:duration-300 hover:text-blue-400 hover:after:w-full"
		>
			Experience
		</a>
		<a
			href="#projects"
			class="relative font-medium transition-colors duration-200 after:absolute after:bottom-[-4px] after:left-0 after:h-[2px] after:w-0 after:bg-blue-400 after:transition-all after:duration-300 hover:text-blue-400 hover:after:w-full"
		>
			Projects
		</a>

		<a
			href="David_Solinsky_resume.pdf"
			target="_blank"
			class="relative font-medium transition-colors duration-200 after:absolute after:bottom-[-4px] after:left-0 after:h-[2px] after:w-0 after:bg-blue-400 after:transition-all after:duration-300 hover:text-blue-400 hover:after:w-full"
		>
			Resume
		</a>
	</nav>

	<div
		id="mobile-menu"
		class="absolute left-0 right-0 top-full {isMenuOpen
			? ''
			: 'hidden'} w-full bg-neutral-800/90 py-2 shadow-lg backdrop-blur-sm md:hidden"
	>
		<nav class="flex flex-col space-y-2 px-4">
			<a
				href="#home"
				class="rounded-lg px-3 py-2 font-medium transition-colors duration-200 hover:bg-neutral-700"
				on:click={() => (isMenuOpen = false)}
			>
				Home
			</a>
			<a
				href="#experience"
				class="rounded-lg px-3 py-2 font-medium transition-colors duration-200 hover:bg-neutral-700"
				on:click={() => (isMenuOpen = false)}
			>
				Experience
			</a>
			<a
				href="#projects"
				class="rounded-lg px-3 py-2 font-medium transition-colors duration-200 hover:bg-neutral-700"
				on:click={() => (isMenuOpen = false)}
			>
				Projects
			</a>
			<a
				href="David_Solinsky_resume.pdf"
				target="_blank"
				class="rounded-lg px-3 py-2 font-medium transition-colors duration-200 hover:bg-neutral-700"
				on:click={() => (isMenuOpen = false)}
			>
				Resume
			</a>
		</nav>
	</div>
</header>
