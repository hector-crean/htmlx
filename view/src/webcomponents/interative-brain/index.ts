import { Selection } from 'd3-selection';
import { css, html, LitElement, svg } from 'lit';
import { createRef, ref } from 'lit-html/directives/ref.js';
import { property } from 'lit/decorators.js';
import { map } from 'lit/directives/map.js';
import defaultBackground from './brain-background.jpg';

// const polygon = () => svg`<polygon class="region" data-interactive="true" points="" fill=""></polygon>`
// const circle = () => svg`<circle data-interactive="true" cx="${}" cy="${}" r="${}" fill="${}"></circle>`
// const path = () => svg`<path data-interactive="true" d="" stroke-width="2" stroke="white" fill="none" stroke-dasharray="" stroke-dashoffset="0"></path>`

// const label = () => svg`<g class="labels"><rect class="region-label" width="" height="" x="" y="" opacity=""></rect><text class="region-label-text" x="" y="" text-anchor="middle" opacity="1">hippocampus</text></g>`

interface Params {
  regions?: Record<string, Region>;
  pathways?: Record<string, Pathway>;
  background?: string;
  interactive?: boolean;
}

interface Region {
  id?: string;
  points?: Point[];
  centroid?: Point;
  fillColor?: string;
  label?: Label;
}

interface Pathway {
  id?: string;
  points?: Point[];
  color?: string;
  width?: number;
  glow?: boolean;
  labels?: Label[];
}

interface Point {
  x: number;
  y: number;
}

interface Label {
  position: Point;
  altDrawMode?: boolean;
  text?: string;
  size?: { width: number; height: number };
  offset?: { x: number; y: number };
}

type Regions = Record<string, Region>;
type Pathways = Record<string, Pathway>;

class InteractiveBrain extends LitElement {
  @property({ type: Object }) params: Params = {};
  @property({ type: Number }) aspectRatio = 1280/720;
  @property({ type: Number }) height = 540;


  @property({ type: Array }) regions: Array<Region> = [];
  @property({ type: Array }) pathways: Array<Pathway> = [];
  @property({ type: String }) background: string = defaultBackground;


  get width() {
    return this.aspectRatio * this.height;
  }


  containerRef = createRef<HTMLDivElement>();
  private svg!: Selection<SVGSVGElement, unknown, null, undefined>;
  
 
  constructor() {
    super();
    this.params = {
      regions: {},
      pathways: {},
      background: defaultBackground,
      interactive: true,
    };
  }

  updated(changedProperties: Map<string, any>) {
    if (changedProperties.has('aspectRatio') || changedProperties.has('height')) {
      this.requestUpdate();
    }
  }

  firstUpdated() {
    const containerEl = this.containerRef.value!;
    const { regions, pathways, background, interactive } = this.params;

  }


  static styles = [
    css`
    :host {
      display: block;
    }
    .container {
      width: 100%;
      height: 100%;
      position: relative;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 20px;
    }
  `];

  render() {
    return html`
      <div ${ref(this.containerRef)} class="container">
        <modal-dialog>
          <div name="content">Hello</div>
        </modal-dialog>
    <svg id="interactive-svg" class="rounded-lg shadow" preserveAspectRatio="xMidYMid meet" data-interactive="true" xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 ${this.width} ${this.height}">
        <defs>
            <filter id="glow">
                <feGaussianBlur stdDeviation="8" result="coloredBlur"></feGaussianBlur>
                <feMerge>
                    <feMergeNode in="coloredBlur"></feMergeNode>
                    <feMergeNode in="SourceGraphic"></feMergeNode>
                </feMerge>
            </filter>
        </defs>
        <g class="zoomable" cursor="grab">
            <g class="bg-images">
              <image x="0" y="0" width="${this.width}" height="${this.height}" xlink:href="/brain-background.jpg" />
            </g>
            <g class="regions">
              ${map(this.regions, (item) => svg`<li>${item}</li>`)}

            </g> 
            <g class="pathways"></g> 
            <g class="centroids"></g> 
            <g class="lines"></g> 
            <g class="labels"></g> 
            <g class="info-area"></g> 
        </g>
    </svg>
</div>
`;
  }
}

customElements.define('interactive-brain', InteractiveBrain);


export { InteractiveBrain };
export type { Label, Pathway, Pathways, Region, Regions };

