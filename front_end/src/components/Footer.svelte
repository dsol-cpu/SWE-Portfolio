<script lang="ts">
	import SocialIcon from './SocialIcon.svelte';
	import EmailContact from './EmailContact.svelte';

	const socialLinks = [
		{
			href: 'https://www.instagram.com/smep_itty/',
			icon: 'https://cdn.builder.io/api/v1/image/assets/TEMP/d39176b56a977509711b171403076c6c77751fec00b288469d52c4edcd8ef200',
			label: 'Instagram Profile',
			size: '18px'
		},
		{
			href: '#',
			icon: 'https://cdn.builder.io/api/v1/image/assets/TEMP/a2d49867247cdaaadecdc4fa25549549e95ad253deda4030595e7ab71f2d23a4',
			label: 'Twitter Profile',
			size: '17px'
		},
		{
			href: '#',
			icon: 'https://cdn.builder.io/api/v1/image/assets/TEMP/9270d2902129f682b1289b4f5b3c16026cece6111f01332a9ca2602ff5ac667d',
			label: 'GitHub Profile',
			size: '20px'
		}
	];

	const emailData = {
		email: 'dsolinsky98@gmail.com',
		icon: 'https://cdn.builder.io/api/v1/image/assets/TEMP/a1164e8c39919f9a28435c6fa8d6973ddf84a873d3f806476e64af1f343553af'
	};
</script>

<footer
	id="contact"
	class="mt-28 flex w-full flex-col items-center justify-center self-stretch bg-zinc-900 px-20 py-24 max-md:mt-10 max-md:max-w-full max-md:px-5 max-md:pb-24"
>
	<div class="flex w-[676px] max-w-full flex-col items-center justify-center">
		<h2 class="text-2xl font-bold tracking-wide text-white">Contact</h2>

		<EmailContact {...emailData} />

		<nav class="mt-10 flex items-center gap-7 max-md:mt-10">
			{#each socialLinks as link}
				<SocialIcon {...link} />
			{/each}
		</nav>
	</div>
</footer>
