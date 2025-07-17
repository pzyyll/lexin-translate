<script setup lang="ts">
const props = defineProps({
	name: { type: String, default: 'Card' },
	switchValue: { type: Boolean, default: false }
});

const emit = defineEmits<{
	clickDelete: void;
	clickEdit: void;
	switchChange: [value: boolean];
}>();

const switchOn = computed({
	get: () => props.switchValue,
	set: (value: boolean) => {
		emit('switchChange', value);
	}
});
</script>

<template>
	<div
		class="du-card du-card-sm bg-base-100 border-base-300 hover:bg-base-300/80 rounded-lg border transition-transform duration-100 hover:-translate-y-[1px]"
	>
		<div class="du-card-body">
			<div class="flex items-center justify-between">
				<div class="flex items-center" name="left">
					<icon-gravity-ui-ellipsis-vertical class="-translate-x-2 cursor-grab" />
					<div class="flex items-center gap-3">
						<div class="du-avatar du-avatar-placeholder select-none">
							<div class="bg-accent text-accent-content w-8 rounded-full">
								<span class="text-xl">
									{{ props.name.charAt(0).toUpperCase() }}
								</span>
							</div>
						</div>
						<h3 class="du-card-title select-none">{{ $t(props.name) }}</h3>
					</div>
				</div>
				<div class="flex items-center justify-end gap-1" name="right">
					<input
						type="checkbox"
						v-model="switchOn"
						class="du-toggle du-toggle-xs du-toggle-success mx-2"
					/>

					<button class="du-btn du-btn-ghost du-btn-xs" @click="$emit('clickEdit')">
						<icon-gravity-ui-pencil-to-square />
					</button>

					<BaseAlertDialog
						:title="$t('delete-channel')"
						:description="$t('delete-channel-description')"
						@confirm="$emit('clickDelete')"
					>
						<button
							class="du-btn du-btn-ghost du-btn-xs hover:bg-error/20 active:bg-error/20 focus:outline-error hover:border-transparent focus:border-transparent active:border-transparent"
						>
							<icon-gravity-ui-trash-bin class="text-error" />
						</button>
					</BaseAlertDialog>
				</div>
			</div>
		</div>
	</div>
</template>
