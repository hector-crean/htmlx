import { LitElement, css, html } from "lit";
import { classMap } from "lit-html/directives/class-map.js";
import { createRef, ref } from "lit-html/directives/ref.js";
import { property } from "lit/decorators.js";

export class ModalDialog extends LitElement {

  @property({ type: Boolean }) open = true;
  @property({ type: String }) title = 'Modal';
  @property({ type: String }) text = '';
  @property({ type: String }) clickAction = '';
  private dialogRef = createRef<HTMLDialogElement>();


  static styles = css`
    :host {
      
    }
    dialog {
  opacity: 0;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  border: none;
  padding: 1rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background-color: #fff;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  transition: opacity 0.3s ease-in-out, transform 0.3s ease-in-out;
}

dialog::backdrop {
  backdrop-filter: blur(10px);
  background-color: rgba(0, 0, 0, 0.75);
  transition: opacity 0.3s ease-in-out;
}

dialog[open] {
  opacity: 1;
  transform: translate(-50%, -50%) scale(1);
}

dialog[open]::backdrop {
  opacity: 0.75;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes fadeOut {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

dialog[open], dialog[open]::backdrop {
  animation: fadeIn 0.3s forwards;
}

dialog[closing], dialog[closing]::backdrop {
  animation: fadeOut 0.3s forwards;
}
    /* @starting-style {
      dialog[open], dialog[open]::backdrop {
        opacity: 0;
      }
    } */
    .dialog h1 {
      margin: 0 0 10px;
    }
   
    .title {
      
    }
    .content_wrapper {
      position: relative;
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      justify-content: center;
    }

    .info_button {
      position: absolute;
      top: 0;
      right: 0;
      padding: 1rem;
      display: flex;
      flex-direction: column;
      align-items: flex-start;
      justify-content: center;
      pointer-events: all;
      z-index: 200;
    }
    .info_button::hover {
      filter: brightness(0.4);
    }

   
  `;

  render() {
    return html`
    <div class=${classMap({info_button: true, clicked: true})} @click=${this.handleClick}>
        <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M7.49991 0.876892C3.84222 0.876892 0.877075 3.84204 0.877075 7.49972C0.877075 11.1574 3.84222 14.1226 7.49991 14.1226C11.1576 14.1226 14.1227 11.1574 14.1227 7.49972C14.1227 3.84204 11.1576 0.876892 7.49991 0.876892ZM1.82707 7.49972C1.82707 4.36671 4.36689 1.82689 7.49991 1.82689C10.6329 1.82689 13.1727 4.36671 13.1727 7.49972C13.1727 10.6327 10.6329 13.1726 7.49991 13.1726C4.36689 13.1726 1.82707 10.6327 1.82707 7.49972ZM8.24992 4.49999C8.24992 4.9142 7.91413 5.24999 7.49992 5.24999C7.08571 5.24999 6.74992 4.9142 6.74992 4.49999C6.74992 4.08577 7.08571 3.74999 7.49992 3.74999C7.91413 3.74999 8.24992 4.08577 8.24992 4.49999ZM6.00003 5.99999H6.50003H7.50003C7.77618 5.99999 8.00003 6.22384 8.00003 6.49999V9.99999H8.50003H9.00003V11H8.50003H7.50003H6.50003H6.00003V9.99999H6.50003H7.00003V6.99999H6.50003H6.00003V5.99999Z"
                fill="currentColor" fill-rule="evenodd" clip-rule="evenodd"></path>
        </svg>
    </div>
    <dialog ${ref(this.dialogRef)}>
        <div class="content_wrapper">
            <slot name="title"></slot>
            <slot name="content"></slot>
        </div>
    </dialog>
    `;
  }

  toggle(){
    this.open = !this.open
    this.handleDialog(this.open)
  }

  handleDialog(open: boolean) {
    const dialogEl = this.dialogRef.value!;
    console.log(dialogEl)
    if(!open){
      dialogEl.show()
    } else {
      dialogEl.close()
    }
  }

 

  handleClick() {
        this.toggle()
        this.requestUpdate();
  }

  // updated(changedProperties: Map<string, any>) {
  //   if(changedProperties.get('open')) {
  //     this.handleDialog(this.open)
  //   }
   
  // }
}

customElements.define('modal-dialog', ModalDialog);

declare global {
  interface HTMLElementTagNameMap {
    "modal-dialog": ModalDialog;
  }
}
