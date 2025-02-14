<script>
	import { resumeModalOpen } from '../lib/stores/resumeModalStore.js';
	import { theme } from '../lib/stores/theme.js';

	$: isDark = $theme === 'dark';
</script>

{#if $resumeModalOpen}
	<div
		class="modal-overlay"
		on:click={() => resumeModalOpen.set(false)}
		on:keydown={(e) => e.key === 'Escape' && resumeModalOpen.set(false)}
		aria-label="Close Resume Modal"
		tabindex="0"
	>
		<div class="modal-content" on:click|stopPropagation>
			<button class="close-button" on:click={() => resumeModalOpen.set(false)}>×</button>
			<iframe src="David_Solinsky_resume.pdf" class="pdf-viewer" title="Resume PDF"></iframe>
		</div>
	</div>
{/if}

<style>
	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.75);
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 1000;
		padding: 0;
		margin: 0;
		opacity: 0;
		animation: fadeIn 0.3s ease-out forwards;
	}

	.modal-content {
		background: var(--modal-bg);
		color: var(--modal-text);
		padding: 20px;
		border-radius: 8px;
		position: relative;
		width: 90%;
		max-width: 800px;
		height: 90vh;
		opacity: 0;
		transform: scale(0.95);
		animation: scaleIn 0.3s ease-out forwards;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}

	@keyframes scaleIn {
		from {
			opacity: 0;
			transform: scale(0.95);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}

	.close-button {
		position: absolute;
		top: 10px;
		right: 10px;
		border: none;
		background: none;
		font-size: 24px;
		cursor: pointer;
		color: var(--close-btn-color);
	}

	.pdf-viewer {
		width: 100%;
		height: 100%;
		border: none;
	}

	:root {
		--modal-bg: white;
		--modal-text: black;
		--close-btn-color: black;
	}

	:root.dark {
		--modal-bg: #1a1a1a;
		--modal-text: white;
		--close-btn-color: white;
	}
</style>
