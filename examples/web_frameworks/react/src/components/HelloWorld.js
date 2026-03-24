import React from 'react';
import { createRoot } from 'react-dom/client';
import HelloWorldReact from './HelloWorld.jsx';
import styles from './HelloWorld.css?raw';

class HelloWorld extends HTMLElement {
    connectedCallback() {
        const shadow = this.attachShadow({ mode: 'open' });

        const style = document.createElement('style');
        style.textContent = styles;
        shadow.appendChild(style);

        const mount = document.createElement('div');
        shadow.appendChild(mount);

        createRoot(mount).render(React.createElement(HelloWorldReact));
    }
}

customElements.define('hello-world', HelloWorld);
