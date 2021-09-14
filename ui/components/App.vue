<template>
    <div class="grid">
        <div class="grid__item" style="grid-column:1/2;grid-row:1/2;">
            <h2>CPU</h2>
            <p><span id="cpu_percent"></span>&nbsp;%</p>
        </div>
        <div class="grid__item" style="grid-column:2/3;grid-row:1/2;">
            <h2>Current Freq.</h2>
            <p><span id="cpu_freq_current"></span>&nbsp;MHz</p>
        </div>
        <div class="grid__item" style="grid-column:3/5;grid-row:1/2;">
            <h2>Disk I/O read</h2>
            <p><span id="disk_io_read_bytes"></span>&nbsp;B</p>
        </div>
        <div class="grid__item" style="grid-column:1/3;grid-row:2/4;">
            <h2>Disk I/O write</h2>
            <p><span id="disk_io_write_bytes"></span>&nbsp;B</p>
        </div>
        <div class="grid__item" style="grid-column:3/4;grid-row:2/3;">
            <h2>Net I/O sent</h2>
            <p><span id="net_io_bytes_sent"></span>&nbsp;B</p>
        </div>
        <div class="grid__item" style="grid-column:3/4;grid-row:3/4;">
            <h2>Net I/O recv</h2>
            <p><span id="net_io_bytes_recv"></span>&nbsp;B</p>
        </div>
        <div class="grid__item" style="grid-column:4/5;grid-row:2/4;">
            <h2>Datetime</h2>
            <p><span id="datetime_now"></span></p>
        </div>
    </div>
</template>

<script>
export default {
}
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

.grid {
    display: grid;
    column-gap: 1rem;
    row-gap: 1rem;
    width: 100%;
    padding: 1rem;
    grid-auto-columns: 1fr;
    grid-auto-rows: 1fr;
}

.grid__item {
    background: #666;
    padding: 1rem;
}
</style>
