<template>
  <div class="tile" :style="{ gridArea }">
    <slot></slot>
  </div>
</template>

<script>
export default {
  props: ["columnStart", "columnEnd", "rowStart", "rowEnd"],
  computed: {
    gridArea: function () {
      return [
        this.rowStart,
        this.columnStart,
        this.rowEnd,
        this.columnEnd,
      ].join("/");
    },
  },
};
</script>

<style scoped>
.tile {
  padding: 1rem;
  display: flex;
  flex-direction: column;
}
</style>
