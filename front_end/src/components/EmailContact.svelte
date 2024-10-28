<script lang="ts">
	export let email: string;
	export let icon: string;
</script>

<div class="mt-6 flex gap-1.5 whitespace-nowrap text-sm font-semibold tracking-wide text-stone-300">
	<img
		loading="lazy"
		src={icon}
		alt="Email Icon"
		class="my-auto aspect-square w-[17px] shrink-0 object-contain"
	/>
	<a href="mailto:{email}" class="basis-auto transition-colors duration-200 hover:text-white">
		{email}
	</a>
</div>
