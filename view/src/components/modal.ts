import { LitElement, css, html } from "lit";
import {
    customElement,
    property
} from "lit/decorators.js";
import { classMap } from "lit/directives/class-map.js";

type ModalDialogProps = {
	open: boolean;
	title: string;
	text: string;
	clickAction: string;
};

@customElement("modal-dialog")
export class ModalDialog extends LitElement {
	@property({ type: Boolean }) open = true;
    

	clickHandler(e: Event) {
		this.open = !this.open;
	}

	static get styles() {
		return css`
      :host {
        font-family: Arial, Helvetica, sans-serif;
      }
      .wrapper {

        opacity: 0;
        position: absolute;
        z-index: 400;
        transition: opacity 0.25s ease-in;
        width: 100dvw;
        height: 100dvh
      }
      .wrapper:not(.open) {
        visibility: hidden;
      }
      .wrapper.open {
        align-items: center;
        display: flex;
        justify-content: center;
        width: 580px;
        height: 580px;
        opacity: 1;
        visibility: visible;
      }
      .overlay {
        background: rgba(0, 0, 0, 0.8);
        height: 100%;
        width: 100%;
        position: relative;
      }
      .dialog {
        background: #ffffff;
        border-radius: 13px;
        max-width: 600px;
        padding: 1rem;
        position: absolute;
      }
      .dialog h1 {
        margin: 0 0 10px;
      }
      .dialog button {
        background-color: #d81e5b;
        color: white;
        width: 100%;
        font-size: 16px;
        padding: 15px 32px;
        border: none;
        border-radius: 10px;
        text-decoration: none;
        display: inline-block;
        margin-top: 10px;
      }`;
	}

	render() {
		return html`
      <div class="${classMap({ wrapper: true, open: this.open })}">
        <div class="overlay" @click="${this.close}"></div>
        <div class="dialog">
          <h1 id="title">${this.title}</h1>
          <div id="content" class="content">Content Here</div>
          <button @click=${this.handleClick}>${this.open}</button>
        </div>
      </div>
    `;
	}

	close() {
		this.open = false;
	}

	handleClick() {
		this.dispatchEvent(new CustomEvent("button-click"));
		this.close();
	}
}

declare global {
	interface HTMLElementTagNameMap {
		"modal-dialog": ModalDialog;
	}
}

