(function () {
    let ws = new WebSocket('ws://localhost:8765');
    ws.onmessage = function (event) {
        let items = JSON.parse(event.data);
        items.forEach(function (item) {
            let el = document.getElementById(item.id);
            if (el) {
                el.innerHTML = item.value;
            }
        });
    }
})();
