import Vue from 'vue'
import App from './components/App.vue'

new Vue({
    el: document.body.appendChild(document.createElement('div')),
    render: h => h(App)
})
