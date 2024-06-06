import { LitElement, css, html } from "lit";
import { classMap } from "lit-html/directives/class-map.js";
import { property } from "lit/decorators.js";


export class ModalDialog extends LitElement {

	@property({type: Boolean}) open = true;
	@property({type: String}) title = 'Modal';
	@property({type: String}) text = '';
	@property({type: String}) clickAction = '';


	static styles = css` 
	    :host {
        
      }
      .wrapper {
        width: 100%;
        height: 100%;
        top: 0;
        bottom: 0;
        left: 0;
        right: 0;
        opacity: 0;
        position: absolute;
        z-index: 999;
        transition: opacity 0.25s ease-in;
      }
      .wrapper:not(.open) {
        visibility: hidden;
      }
      .wrapper.open {
        align-items: center;
        display: flex;
        justify-content: center;
       
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
      }
      #title {
        color: red;
      }
      #content {
        color: red;
      }
      `;

	
  
	render() {
	  return html`
		  <div class="${classMap({wrapper: true, open: this.open})}">
        <div class="overlay" @click="${this.close}"></div>
        <div class="dialog">
          <h1 id="title">${this.title}</h1>
          <div id="content" class="content">
            <slot name="content"></slot>
          </div>
          <button @click=${this.handleClick}>${this.clickAction}</button>
        </div>
      </div>
	  `;
	}

	close() {
		this.open = false;
	  }
	
	  handleClick() {
		this.dispatchEvent(new CustomEvent('button-click'));
		this.close();
	  }

}

customElements.define('modal-dialog', ModalDialog);


declare global {
	interface HTMLElementTagNameMap {
		"modal-dialog": ModalDialog;
	}
}
