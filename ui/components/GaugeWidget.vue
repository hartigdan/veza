<template>
  <div class="gauge">
    <svg
      viewBox="-10 -20 20 20"
      width="200"
      height="100"
      xmlns="http://www.w3.org/2000/svg"
    >
      <g :style="style">
        <circle cx="0" cy="0" r="3" />
        <line x1="0" y1="0" x2="-20" y2="0" stroke="black" />
      </g>
    </svg>
    <p>{{ value }}</p>
  </div>
</template>

<script>
export default {
  props: ["value", "min", "max"],
  computed: {
    style: function () {
      return {
        transform: "rotate(" + Number(this.value) * 1.8 + "deg)",
      };
    },
  },
};
</script>

<style scoped>
g {
  transition: all 500ms ease-in-out;
}
</style>
