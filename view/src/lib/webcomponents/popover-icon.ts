import { html, LitElement } from "lit";




class PopoverIcon extends LitElement {

    protected render() {
        html`
        <button popover-target="my-popover"></button>
        <div id="my-popover" popover>Popover</div>
        `
    }
}