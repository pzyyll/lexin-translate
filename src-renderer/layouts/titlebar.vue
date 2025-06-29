<script setup lang="ts">
useHead({
  htmlAttrs: {
    class: "bg-base-200 h-screen",
  },
  bodyAttrs: {
    class: "bg-base-200 h-screen",
  },
});
</script>

<template>
  <div class="h-screen flex flex-col">
    <WinTitleBar />
    <div class="w-full flex-1">
      <slot />
    </div>
  </div>
</template>
