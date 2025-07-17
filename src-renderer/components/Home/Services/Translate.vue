<script setup lang="ts">
import { useSortable } from '@vueuse/integrations/useSortable';

const translateApiStore = useTranslateApiStore();

const { channels } = storeToRefs(translateApiStore);

const el = useTemplateRef<HTMLElement>('el');
useSortable(el, channels, {
	animation: 150,
	handle: '.cursor-grab'
});

const isOpenDialog = ref(false);
const openDialogChannel = ref<Translate.Channel>();

async function onClickEdit(id: string) {
	console.log('Editing channel with ID:', id);
	// await translateApiStore.OpenChannelConfigDialog(id);
	const channel = await translateApiStore.GetChannelById(id);
	if (channel) {
		openDialogChannel.value = channel;
		console.log('Open dialog with channel data:', openDialogChannel.value);
		isOpenDialog.value = true;
	}
}

async function onDeleteChannel(id: string) {
	console.log('Deleting channel with ID:', id);
	await translateApiStore.DeleteChannel(id);
}

function onSwitchChange(id: string, value: boolean) {
	translateApiStore.GetChannelById(id).then((value) => {
		console.log(value);
	});

	console.log('Switch changed for channel ID:', id, 'to value:', value);
	translateApiStore.UpdateChannel({
		id,
		enable: value
	});
}

watch(
	() => isOpenDialog.value,
	(isOpen) => {
		if (!isOpen) {
			openDialogChannel.value = undefined;
		}
	}
);

onMounted(() => {
	for (const channel of channels.value) {
		console.log('Channel:', channel);
	}
});
</script>

<template>
	<div class="flex w-full flex-col gap-2">
		<div ref="el" class="flex w-full flex-col gap-2">
			<div v-for="channel in channels" :key="channel.id">
				<HomeComponentCard
					:name="channel.name"
					:switchValue="channel.enable"
					@click-delete="onDeleteChannel(channel.id)"
					@click-edit="onClickEdit(channel.id)"
					@switch-change="(value: boolean) => onSwitchChange(channel.id, value)"
				/>
			</div>
		</div>
		<HomeServicesTranslateConfigDialog
			v-model:is-open="isOpenDialog"
			:channelData="openDialogChannel"
			@edit="onClickEdit"
		>
			<HomeComponentNewCard text="new-translate-api" />
		</HomeServicesTranslateConfigDialog>
	</div>
</template>
