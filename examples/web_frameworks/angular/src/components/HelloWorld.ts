import 'zone.js';
import { Component, signal } from '@angular/core';
import { createApplication } from '@angular/platform-browser';
import { createCustomElement } from '@angular/elements';

@Component({
    standalone: true,
    selector: 'hello-world-ng',
    template: `
        <div>
            <input [value]="name()" (input)="name.set($any($event).target.value)" placeholder="Enter your name" />
            @if (name()) {
                <p>Hello {{ name() }}!</p>
            }
        </div>
    `,
    styles: [`
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
    `],
})
export class HelloWorldComponent {
    name = signal('');
}

createApplication().then((appRef) => {
    const HelloWorld = createCustomElement(HelloWorldComponent, {
        injector: appRef.injector,
    });
    customElements.define('hello-world', HelloWorld);
});
