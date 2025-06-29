<script setup lang="ts">
interface Option {
  name: string;
  code: string;
}

const selected = defineModel<Option>();

const props = defineProps<{
  options?: Option[];
  extra?: string;
}>();
</script>
<template>
  <USelectMenu
    v-model="selected"
    :items="props.options"
    :ui="{
      trailingIcon: 'group-data-[state=open]:rotate-180 transition-transform duration-200'
    }"
    labelKey="name"
    :filter-fields="['name', 'code']"
    class="w-40"
    size="sm"
    variant="none"
  >
  </USelectMenu>
</template>

<style scoped>
@reference "~/assets/css/main.css"
.unwrapped p {
  @apply whitespace-nowrap;
}
</style>
