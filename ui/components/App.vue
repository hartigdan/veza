<template>
  <div class="app">
    <Tile
      v-for="(tile, index) in tiles"
      :key="index"
      :column-start="tile.columnStart"
      :column-end="tile.columnEnd"
      :row-start="tile.rowStart"
      :row-end="tile.rowEnd"
    >
      <h2>{{ tile.title }}</h2>
      <component :is="tile.type" v-bind="tile" />
    </Tile>
  </div>
</template>

<script>
import Tile from "./Tile.vue";
import TextWidget from "./TextWidget.vue";
import GaugeWidget from "./GaugeWidget.vue";

export default {
  created: function () {
    let ws = new WebSocket("ws://localhost:8765");
    ws.onmessage = (event) => {
      let items = JSON.parse(event.data);
      this.tiles.forEach((tile) => {
        let match = items.find((item) => item.id == tile.id);
        if (match) {
          tile.value = match.value;
        }
      });
    };
  },
  data: function () {
    return {
      tiles: [
        {
          id: "cpu_percent",
          type: "GaugeWidget",
          title: "CPU",
          value: 0,
          min: 0,
          max: 100,
          columnStart: 1,
          columnEnd: 2,
          rowStart: 1,
          rowEnd: 2,
        },
        {
          id: "cpu_freq_current",
          type: "TextWidget",
          title: "Current Freq.",
          value: "",
          columnStart: 2,
          columnEnd: 3,
          rowStart: 1,
          rowEnd: 2,
        },
        {
          id: "disk_io_read_bytes",
          type: "TextWidget",
          title: "Disk I/O read",
          value: "",
          columnStart: 3,
          columnEnd: 5,
          rowStart: 1,
          rowEnd: 2,
        },
        {
          id: "disk_io_write_bytes",
          type: "TextWidget",
          title: "Disk I/O write",
          value: "",
          columnStart: 1,
          columnEnd: 3,
          rowStart: 2,
          rowEnd: 4,
        },
        {
          id: "net_io_bytes_sent",
          type: "TextWidget",
          title: "Net I/O sent",
          value: "",
          columnStart: 3,
          columnEnd: 4,
          rowStart: 2,
          rowEnd: 3,
        },
        {
          id: "net_io_bytes_recv",
          type: "TextWidget",
          title: "Net I/O recv",
          value: "",
          columnStart: 3,
          columnEnd: 4,
          rowStart: 3,
          rowEnd: 4,
        },
        {
          id: "datetime_now",
          type: "TextWidget",
          title: "Datetime",
          value: "",
          columnStart: 4,
          columnEnd: 5,
          rowStart: 2,
          rowEnd: 4,
        },
      ],
    };
  },
  components: {
    GaugeWidget,
    TextWidget,
    Tile,
  },
};
</script>

<style>
*,
*::before,
*::after {
  font-size: 1rem;
  margin: 0;
  padding: 0;
}

h2 {
  font-size: 1.5rem;
}

p {
  margin: 1em 0;
}

body {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  display: flex;
  color: #fff;
}

.app {
  display: grid;
  column-gap: 1rem;
  row-gap: 1rem;
  width: 100%;
  padding: 1rem;
  grid-auto-columns: 1fr;
  grid-auto-rows: 1fr;
}
</style>
