class SVGContainer extends HTMLElement {
  constructor() {
    super();
    const shadowRoot = this.attachShadow({mode: 'closed'});

   


    shadowRoot.append(...this.childNodes);
  }
}

customElements.define('svg-container', SVGContainer);