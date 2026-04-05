import { LitElement, html, css } from 'lit';

class HelloWorld extends LitElement {
    static properties = {
        name: { type: String },
    };

    static styles = css`
        :host {
            display: block;
        }

        div {
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }

        input {
            box-sizing: border-box;
            padding: 0.5rem 0.75rem;
            border: 1px solid #ccc;
            border-radius: 6px;
            font-size: 1rem;
            font-family: inherit;
            width: 100%;
            outline: none;
            transition: border-color 0.15s;
        }

        input:focus {
            border-color: #555;
        }

        p {
            font-size: 1.25rem;
            font-weight: 600;
            margin: 0;
            color: #1a1a1a;
        }
    `;

    constructor() {
        super();
        this.name = '';
    }

    render() {
        return html`
            <div>
                <input
                    .value=${this.name}
                    placeholder="Enter your name"
                    @input=${(e) => (this.name = e.target.value)}
                />
                ${this.name ? html`<p>Hello ${this.name}!</p>` : ''}
            </div>
        `;
    }
}

customElements.define('hello-world', HelloWorld);
