import { axisBottom, axisLeft } from 'd3-axis';
import { format } from 'd3-format';
import { scaleBand, scaleLinear, scaleOrdinal } from 'd3-scale';
import { select } from 'd3-selection';
import { stack } from 'd3-shape';
import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { fromEvent } from 'rxjs';
import { debounceTime } from 'rxjs/operators';
import pattern from './textures';

type StackedBarChartData<Datum extends Record<string, number> = Record<string, number>> = {
  group: string;
  values: Datum;
};

declare global {
    interface HTMLElementTagNameMap {
      'stacked-bar-chart': StackedBarChart;
    }
  }


@customElement('stacked-bar-chart')
class StackedBarChart extends LitElement {
  @property({ type: Array }) data: StackedBarChartData[] = [];
  @property({ type: Object }) margin = { top: 20, right: 30, bottom: 40, left: 200 };

  private svg;
  private scaleY;
  private scaleX;
  private colorScale;
  private size;
  private resizeObservable$;

  static styles = css`
    :host {
      display: block;
      position: relative;
    }
    .tooltip {
      position: absolute;
      z-index: 10;
      visibility: hidden;
      padding: 10px;
      background: rgba(0, 0, 0, 0.6);
      border-radius: 4px;
      color: #fff;
    }
  `;

  firstUpdated() {
    this.initializeChart();
    this.resizeObservable$ = fromEvent(window, 'resize').pipe(debounceTime(200));
    this.resizeObservable$.subscribe(() => this.onResize());
  }

  initializeChart() {
    const container = this.shadowRoot?.getElementById('chart-container') as HTMLElement;
    this.size = this.calculateSize(container, this.margin);
    this.createScales();
    this.createSVG(container);
    this.drawChart();
  }

  createScales() {
    const groups = this.data.map(d => d.group);
    const subgroups = Object.keys(this.data[0].values);
    this.scaleY = scaleBand().domain(groups).padding(0.1).range([0, this.size.innerHeight]);
    const xs = this.data.flatMap(datum => Object.values(datum.values).reduce((sum, x) => sum + x, 0));
    this.scaleX = scaleLinear().domain([0, 100]).nice().range([0, this.size.innerWidth]);
    this.colorScale = scaleOrdinal().domain(subgroups).range(["#7ea8cb", "#8EC1D3", "#9FD6DB", "#AFE2DD", "#C0E9DD", "#D2F0E2", "#E4F6EA"]);
  }

  createSVG(container: HTMLElement) {
    this.svg = select(container).append('svg')
      .attr('width', '100%')
      .attr('height', '100%')
      .attr('viewport', `0 0 ${this.size.innerWidth} ${this.size.innerHeight}`)
      .attr('data-interactive', true);
  }

  drawChart() {
    const defs = this.svg.append("defs");
    const flowFilter = defs.append('filter').attr("id", "glow");
    flowFilter.append("feGaussianBlur").attr("stdDeviation", "3.5").attr("result", "coloredBlur");
    const feMerge = flowFilter.append("feMerge");
    feMerge.append("feMergeNode").attr("in", "coloredBlur");
    feMerge.append("feMergeNode").attr("in", "SourceGraphic");

    const pattern1 = pattern.lines().size(8).strokeWidth(2).stroke("#7ea8cb");
    this.svg.call(pattern1);

    const vizLayer = this.svg.append('g').attr('transform', `translate(${this.margin.left},${this.margin.top})`).classed('viz', true);
    const graphLayer = vizLayer.append("g").classed("graphLayer", true);
    graphLayer.append('rect').attr('width', this.size.innerWidth).attr('height', this.size.innerHeight).attr('fill', 'transparent').attr('data-interactive', true);

    const xAxisContainer = vizLayer.append('g');
    const xAxis = axisBottom(this.scaleX).tickFormat(n => format(".0%")(n.valueOf() / 100));
    xAxisContainer.attr('class', 'x-axis').attr('transform', `translate(0,${this.size.innerHeight})`).call(xAxis);

    const yAxisContainer = vizLayer.append('g');
    const yAxis = axisLeft(this.scaleY).ticks(10, 's');
    yAxisContainer.attr('class', 'y-axis').call(yAxis);

    const ds = this.data.map(data => ({ ...data.values, group: data.group }));
    const stackedData = stack().keys(Object.keys(this.data[0].values))(ds);

    const stacks = graphLayer.selectAll("g").data(stackedData).join("g").attr("fill", d => pattern1.url());
    const bars = stacks.selectAll("rect").data(D => D).join('rect')
      .attr('data-interactive', true)
      .attr("x", d => this.scaleX(d[0]))
      .attr("y", d => this.scaleY(d.data.group)!)
      .attr("height", d => this.scaleY.bandwidth())
      .attr("width", d => this.scaleX(d[1]) - this.scaleX(d[0]));

    bars.on("pointerover", function () { select(this).style("cursor", "pointer"); })
      .on("pointerdown", function () {
        bars.attr('filter', null);
        select(this).attr('filter', "url(#glow)");
      });

    graphLayer.select('rect').on("pointerdown", function () {
      bars.attr('filter', null);
    });
  }

  onResize() {
    const container = this.shadowRoot?.getElementById('chart-container') as HTMLElement;
    this.size = this.calculateSize(container, this.margin);
    this.scaleX.range([0, this.size.innerWidth]);
    this.scaleY.range([this.size.innerHeight, 0]);
    this.svg.attr('width', '100%').attr('height', '100%').attr('viewport', `0 0 ${this.size.innerWidth} ${this.size.innerHeight}`);
    this.drawChart();
  }

  calculateSize(containerEl: HTMLElement, margin: any) {
    const rect = containerEl.getBoundingClientRect();
    return {
      innerWidth: rect.width - margin.left - margin.right,
      innerHeight: rect.height - margin.top - margin.bottom,
      outerWidth: rect.width,
      outerHeight: rect.height
    };
  }

  render() {
    return html`<div id="chart-container"></div>`;
  }
}

// Sample data and ord logic
const stackedData: Array<StackedBarChartData> = [
  { group: 'Self-harm', values: { baseline: 60, uncertainty: 10 } },
  { group: 'Anxiety', values: { baseline: 40, uncertainty: 4 } },
  { group: 'Major Depressive Disorder', values: { baseline: 38, uncertainty: 7 } }
];

const barChartOrd = {
  equals: (x, y) => x === y,
  compare: (first, second) => first.values > second.values ? 1 : first.values < second.values ? -1 : 0
};

export { StackedBarChart, barChartOrd, stackedData };

