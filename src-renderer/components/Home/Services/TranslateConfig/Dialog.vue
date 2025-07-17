<script setup lang="ts">
const channelTypes = shallowRef([
	{ value: TranslateApiType.OpenAI, label: 'openai-nt' }
	// { value: TranslateApiType.DeepL, label: 'deepl-nt' },
	// { value: TranslateApiType.Google, label: 'google-nt' },
	// { value: TranslateApiType.Baidu, label: 'baidu-nt' }
]);

function getSelectedTypeLabel(value: string): string {
	const type = channelTypes.value.find((type) => type.value === value);
	return type ? type.label : '';
}

const translateApiStore = useTranslateApiStore();

const channelRef = useTemplateRef<any>('channelRef');

const isOpen = defineModel<boolean>('isOpen', { default: false });

const props = defineProps<{
	channelData?: Translate.Channel;
}>();

const defaultData = ref<Translate.Channel>({
	id: '',
	name: '',
	api_type: '',
	api_config: {},
	enable: false
});

const data = computed({
	get: () => props.channelData || defaultData.value,
	set: (value) => {
		defaultData.value = value;
	}
});

const customName = computed({
	get: () => data.value.name,
	set: (value) => {
		data.value.name = value;
	}
});

const apiType = computed({
	get: () => data.value.api_type,
	set: (value) => {
		console.log('Setting API type:', value);
		data.value.api_type = value;
	}
});

const showTest = computed(() => {
	if (!apiType.value) return false;
	if (channelRef.value) {
		return typeof channelRef.value.TestConnection === 'function';
	}
});

const hasTriggerTest = ref(false);
const testResult = ref(false);
const isTesting = ref(false);
const channelHasError = ref(false);

async function TestConnection() {
	// Implement the logic to test the connection to the selected API
	// This could involve making a request to the API with the provided credentials

	if (isTesting.value) {
		console.warn('A test is already in progress. Please wait.');
		return;
	}

	hasTriggerTest.value = true;
	isTesting.value = true;

	try {
		// Simulate an API call
		testResult.value = await channelRef.value?.TestConnection?.();
	} catch (error) {
		testResult.value = false; // Assume the test failed
	} finally {
		isTesting.value = false;
	}
}

async function handleSave() {
	// Implement the logic to save the channel configuration
	// This could involve sending the data to a backend or updating a store

	console.log('Saving channel configuration:', data.value);

	if (!apiType.value) {
		console.warn('Please select an API type before saving.');
		return;
	}

	if (data.value.id === '') {
		translateApiStore.AddChannel(data.value);
	} else {
		translateApiStore.UpdateChannel(data.value);
	}

	isOpen.value = false; // Close the dialog after saving
}

function resetDialog() {
	hasTriggerTest.value = false;
	testResult.value = false;
	isTesting.value = false;
	channelHasError.value = false;
}

watch(apiType, () => {
	resetDialog();
});

watch(isOpen, (newValue) => {
	if (newValue) {
		// Reset the selected API type when the dialog opens
		resetDialog();
	}
});

onErrorCaptured((err, instance, info) => {
	console.error('Error captured in TranslateConfigDialog:', err);
	// Optionally, you can set an error state or log it to an external service
	channelHasError.value = true;
	return false; // Prevent the error from propagating further
});
</script>

<template>
	<DialogRoot v-model:open="isOpen">
		<DialogTrigger asChild>
			<slot> </slot>
		</DialogTrigger>
		<DialogPortal>
			<Transition name="fade">
				<DialogOverlay class="fixed inset-0 z-10 bg-black/50"></DialogOverlay>
			</Transition>
			<Transition name="content-fade">
				<DialogContent
					class="bg-base-100 z-100 fixed left-[50%] top-[50%] flex max-h-[85vh] w-[60vw] min-w-64 max-w-sm translate-x-[-50%] translate-y-[-50%] flex-col gap-2 rounded-xl px-3 py-2 shadow-xl"
					v-on:interactOutside="$event.preventDefault()"
				>
					<DialogTitle class="text-base-content text-lg">{{ $t('channel-config') }}</DialogTitle>
					<div class="flex flex-col gap-2 overflow-y-auto px-1">
						<fieldset class="du-fieldset">
							<div class="flex flex-col gap-1.5">
								<label class="du-label">{{ $t('api-type') }}</label>
								<select class="du-select du-select-xs" v-model="apiType" :disabled="data.id !== ''">
									<option value="" disabled selected>{{ $t('channel-select') }}</option>
									<option v-for="type in channelTypes" :key="type.value" :value="type.value">
										{{ $t(type.label) }}
									</option>
								</select>
							</div>

							<Transition name="config-fade">
								<div class="flex flex-col gap-1.5" v-if="apiType !== ''">
									<label class="du-label">{{ $t('custom-name') }}</label>
									<input
										type="text"
										class="du-input du-input-xs"
										:placeholder="$t(getSelectedTypeLabel(apiType))"
										v-model="customName"
									/>
								</div>
							</Transition>
							<!-- TODO with plugins -->
							<Transition name="config-fade">
								<div v-if="channelHasError" class="text-red-500">Loading Channel Plugin Error</div>
								<template v-else>
									<HomeServicesTranslateConfigChannelOpenAI
										v-model="data"
										ref="channelRef"
										:name="customName"
										v-if="apiType === TranslateApiType.OpenAI && !channelHasError"
									/>
								</template>
							</Transition>
						</fieldset>
					</div>
					<div class="flex items-center justify-between gap-2 px-1">
						<div class="flex items-center gap-2">
							<div class="flex items-center gap-1">
								<a class="du-link text-sm text-green-500" v-if="showTest" @click="TestConnection">{{
									$t('test-connection')
								}}</a>
								<Transition name="test-fade" mode="out-in">
									<span
										class="du-loading du-loading-spinner du-loading-xs"
										v-if="showTest && hasTriggerTest && isTesting"
									></span>
									<icon-gravity-ui-check
										v-else-if="showTest && hasTriggerTest && !isTesting && testResult"
										class="text-green-500"
									/>
									<icon-gravity-ui-xmark
										v-else-if="showTest && hasTriggerTest && !isTesting && !testResult"
										class="text-red-500"
									/>
								</Transition>
							</div>
						</div>
						<div class="flex items-center gap-2">
							<DialogClose asChild>
								<button class="du-btn du-btn-ghost du-btn-sm">{{ $t('cancel') }}</button>
							</DialogClose>

							<button class="du-btn du-btn-sm" @click="handleSave" :disabled="!apiType">
								{{ $t('save') }}
							</button>
						</div>
					</div>
				</DialogContent>
			</Transition>
		</DialogPortal>
	</DialogRoot>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
	transition: opacity 0.2s ease-in-out;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}

.content-fade-enter-active,
.content-fade-leave-active {
	transition:
		opacity 0.1s ease-in-out,
		transform 0.1s ease-in-out;
}

.content-fade-enter-from,
.content-fade-leave-to {
	opacity: 0;
	transform: translateY(-10px);
}

.test-fade-enter-active,
.test-fade-leave-active {
	transition: all 0.15s ease-in-out;
}
.test-fade-enter-from,
.test-fade-leave-to {
	opacity: 0;
	transform: scale(0.4);
}

.config-fade-enter-active,
.config-fade-leave-active {
	transition: all 0.2s ease-in;
}
.config-fade-enter-from,
.config-fade-leave-to {
	opacity: 0;
	height: 0;
}
</style>
