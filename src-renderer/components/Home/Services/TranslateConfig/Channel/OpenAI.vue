<script lang="ts" setup>
interface Message {
	role: 'user' | 'assistant' | 'system';
	content: string;
}

interface Prompt {
	id: string;
	name: string;
	enable: boolean;
	messages: Message[];
}

interface Config {
	apiKey: string;
	baseUrl?: string;
	model: string;
	temperature: number;
	prompts?: Prompt[];
}

const props = defineProps<{
	name?: string;
}>();

const data = defineModel<Translate.Channel>({ required: true });

const config = computed<Config>(() => {
	return data.value.api_config;
});

async function TestConnection(): Promise<boolean> {
	if (!config.value.apiKey) {
		console.warn('API key is required for testing connection.');
		return false;
	}

	console.log('Testing connection with:', {
		apiKey: config.value.apiKey,
		baseUrl: config.value.baseUrl,
		model: config.value.model,
		temperature: config.value.temperature
	});
	return true;
}

watch(
	() => props.name,
	(newName) => {
		if (newName) {
			data.value.name = newName;
		}
	}
);

defineExpose({
	TestConnection
});

onMounted(() => {
	// Initialize config with default values if not provided
	if (data.value.id === '') {
		data.value.name = props.name || 'OpenAI';
		data.value.api_config = {
			apiKey: '',
			baseUrl: 'https://api.openai.com/v1',
			model: 'gpt-4.1-mini',
			temperature: 0.7,
			prompts: []
		};
	}
});
</script>

<template>
	<div class="flex flex-col gap-1.5">
		<label class="du-label">{{ $t('api-key') }}</label>
		<label class="du-input du-input-xs">
			<input type="password" placeholder="sk-xxx" v-model="config.apiKey" />
		</label>

		<label class="du-input du-input-xs">
			<span>URL</span>
			<input type="text" placeholder="https://api.openai.com/v1" v-model="config.baseUrl" />
			<span class="du-badge du-badge-neutral du-badge-xs">Optional</span>
		</label>

		<label class="du-label">{{ $t('model') }}</label>
		<input
			type="text"
			class="du-input du-input-xs"
			placeholder="gpt-4.1-mini"
			v-model="config.model"
		/>

		<label class="du-label">{{ $t('temperature') }}</label>
		<div class="flex items-center gap-1">
			<input
				type="range"
				min="0.0"
				max="2.0"
				step="0.1"
				v-model="config.temperature"
				class="du-range du-range-xs text-green-500"
			/>
			<span class="w-6 select-none text-right">{{ config.temperature }}</span>
		</div>

		<label class="du-label">{{ $t('prompt') }}</label>
		<div class="border-base-300 flex flex-col gap-1 rounded-lg border p-1">
			<div class="flex flex-wrap gap-2">
				<div class="du-join rounded-field" v-for="(prompt, index) in config.prompts" :key="index">
					<button class="du-btn du-btn-xs du-join-item">{{ prompt.name }}</button>
					<button class="du-btn du-btn-xs du-join-item">
						<icon-gravity-ui-pencil-to-square />
					</button>
					<button class="du-btn du-btn-xs du-join-item">
						<icon-gravity-ui-xmark />
					</button>
				</div>
			</div>
			<div class="flex items-center justify-start gap-0.5">
				<button class="du-btn du-btn-ghost du-btn-xs">
					<icon-gravity-ui-circle-plus class="opacity-80" />
				</button>
				<button class="du-btn du-btn-ghost du-btn-xs">
					<icon-gravity-ui-arrow-up-from-square class="opacity-80" />
				</button>
			</div>
		</div>
	</div>
</template>
