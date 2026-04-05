import { defineCustomElement } from 'vue';
import HelloWorldVue from './HelloWorld.vue';
import styles from './HelloWorld.css?raw';

const HelloWorld = defineCustomElement({
    ...HelloWorldVue,
    styles: [styles],
});

customElements.define('hello-world', HelloWorld);
