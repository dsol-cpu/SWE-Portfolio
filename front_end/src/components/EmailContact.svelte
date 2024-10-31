<script lang="ts">
	interface EmailProps {
		email: string;
		icon: string;
	}

	export let email: EmailProps['email'];
	export let icon: EmailProps['icon'];
</script>

<div class="flex w-full items-center justify-center gap-2 lg:justify-start">
	<img src={icon} alt="Email Icon" loading="lazy" class="h-[17px] w-[17px] object-contain" />
	<a
		href="mailto:{email}"
		class="text-sm font-semibold tracking-wide text-stone-300 transition-colors duration-200 hover:text-white"
	>
		{email}
	</a>
</div>

<style lang="postcss">
	/* Optional: Add any component-specific styles here */
	a:focus-visible {
		outline: 2px solid white;
		outline-offset: 2px;
	}
</style>
