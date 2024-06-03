import { easeSinInOut } from "d3-ease";
import { scaleLinear } from "d3-scale";
import { pointer, select, Selection, selection } from "d3-selection";
import { curveNatural, line } from "d3-shape";

import defaultBackground from "./brain-background.jpg";

import selection_interrupt from "../../../node_modules/d3-transition/src/selection/interrupt";
import selection_transition from "../../../node_modules/d3-transition/src/selection/transition";
selection.prototype.interrupt = selection_interrupt;
selection.prototype.transition = selection_transition;



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

type Regions = Record<string, Region>
type Pathways = Record<string, Pathway>

class InteractiveBrain {
  private svg: Selection<SVGSVGElement, unknown, HTMLElement, any>;
  private zoomableContainer: Selection<SVGGElement, unknown, HTMLElement, any>;
  private layer0: Selection<SVGGElement, unknown, HTMLElement, any>;
  private pathwaysGroup: Selection<SVGGElement, unknown, HTMLElement, any>;
  private layer1: Selection<SVGGElement, unknown, HTMLElement, any>;
  private layer2: Selection<SVGGElement, unknown, HTMLElement, any>;
  private layer3: Selection<SVGGElement, unknown, HTMLElement, any>;
  private layer4: Selection<SVGGElement, unknown, HTMLElement, any>;
  private lastSelected: string | null = null;
  private regions: Regions;
  private pathways: Pathways;

  constructor(element: SVGElement, params: Params) {
    const defaults: Params = {
      regions: {},
      pathways: {},
      background: defaultBackground,
      interactive: true,
    };

    params = { ...defaults, ...params };

    this.regions = params.regions || {};
    this.pathways = params.pathways || {};

    const width = 960;
    const height = 540;

    this.svg = select(element).attr("data-interactive", true);

    this.zoomableContainer = this.svg.append("g").classed("zoomable", true).attr("cursor", "grab");;

    const zoomed = ({ transform }) => {
      this.zoomableContainer.attr("transform", transform);
    }




    // this.svg.call(zoom()
    //   .extent([[0, 0], [width, height]])
    //   .scaleExtent([1, 20])
    //   .on("zoom", zoomed));


    this.layer0 = this.zoomableContainer.append("g").classed("regions", true);
    this.pathwaysGroup = this.zoomableContainer.append("g").classed("pathways", true);
    this.layer1 = this.zoomableContainer.append("g").classed("centroids", true);
    this.layer2 = this.zoomableContainer.append("g").classed("lines", true);
    this.layer3 = this.zoomableContainer.append("g").classed("labels", true);
    this.layer4 = this.zoomableContainer.append("g").classed("infoArea", true);

    const scaleX = scaleLinear().domain([0, 960]).range([0, width]);
    const scaleY = scaleLinear().domain([0, 540]).range([0, height]);

    this.svg.on("pointerdown", function (event) {
      const [x, y] = pointer(event);
    });

    this.layer0
      .append("image")
      .attr("x", 0)
      .attr("y", 0)
      .attr("width", width)
      .attr("height", height)
      .attr("xlink:href", params.background ?? defaultBackground);

    this.layer0
      .selectAll("polygon")
      .data(Object.keys(this.regions))
      .enter()
      .append("polygon")
      .attr("class", "region")
      .attr("data-interactive", true)
      .attr("points", (d) => {
        const region = this.regions[d];
        if (region.points) {
          return region.points
            .map((p) => `${scaleX(p.x)},${scaleY(p.y)}`)
            .join(" ");
        }
        return "";
      })
      .attr("fill", (d) => this.regions[d]?.fillColor ?? "")
      .on("pointerdown", (event, d) => {
        if (params.interactive) {
          this.highlightRegions([d]);
        }
      });

    const curve = line().curve(curveNatural);

    this.pathwaysGroup
      .selectAll("path")
      .data(Object.keys(this.pathways))
      .enter()
      .append("path")
      .attr("data-interactive", true)
      .attr("class", "pathway")
      .attr("d", (d) => {
        const pathway = this.pathways[d];
        pathway.id = d;
        const points = pathway.points?.map((p) => [p.x, p.y]);
        return points ? curve(points as [number, number][]) : "";
      })
      .attr("stroke", "none")
      .attr("fill", "none");

    const defs = this.svg.append("defs");
    const filter = defs.append("filter").attr("id", "glow");
    filter.append("feGaussianBlur").attr("stdDeviation", "8").attr("result", "coloredBlur");
    const feMerge = filter.append("feMerge");
    feMerge.append("feMergeNode").attr("in", "coloredBlur");
    feMerge.append("feMergeNode").attr("in", "SourceGraphic");
  }

  clearLabels() {
    const priorPath = this.layer2.selectAll("path");
    if (!priorPath.empty()) {
      priorPath.each(function () {
        const path = select(this);
        const priorLength = path.node().getTotalLength();
        path
          .attr("stroke-dasharray", `${priorLength} ${priorLength}`)
          .attr("stroke-dashoffset", 0)
          .attr("data-interactive", true)
          .transition()
          .delay(500)
          .duration(500)
          .ease(easeSinInOut)
          .attr("stroke-dashoffset", priorLength)
          .remove();
      });
    }

    this.layer3.selectAll("rect").transition().delay(250).duration(500).attr("opacity", 0).remove();
    this.layer3.selectAll("text").transition().duration(500).attr("opacity", 0).remove();
  }

  clearRegionsAndPathways() {
    this.svg.selectAll("polygon").classed("active", false);

    const pathways = this.pathwaysGroup.selectAll("path");
    if (!pathways.empty()) {
      pathways.transition().duration(500).ease(easeSinInOut).attr("stroke-width", 0);
    }
  }

  clearCentroids() {
    this.layer1.selectAll("circle").remove();
  }

  isAlreadySelected(data: string[]) {
    const selected = data.join("-");
    if (this.lastSelected === selected) {
      return true;
    }
    this.lastSelected = selected;
    return false;
  }

  highlightRegions(regions: string[]) {
    if (this.isAlreadySelected(regions)) {
      return;
    }

    this.clearRegionsAndPathways();
    this.clearLabels();

    if (this.layer1.selectAll("circle").size() > 1 || regions.length > 1) {
      this.clearCentroids();
    }

    if (!Array.isArray(regions)) {
      regions = [regions];
    }

    regions.forEach((regionKey) => {
      const region = this.regions[regionKey];
      if (!region) return;


      const centroid = this.layer1.select("circle");
      if (centroid.empty() || regions.length > 1) {
        this.layer1
          .append("circle")
          .attr("data-interactive", true)
          .attr("cx", region.centroid?.x || 0)
          .attr("cy", region.centroid?.y || 0)
          .attr("r", 5)
          .attr("fill", "white");
      } else {
        centroid.transition().duration(500).attr("cx", region.centroid?.x || 0).attr("cy", region.centroid?.y || 0);
      }

      if (region.label) {
        const lineFunction = line<Point>()
          .x((d) => d.x)
          .y((d) => d.y);

        const sOffsetX = region.label.position.x < region.centroid?.x ? -10 : 10;
        const sOffsetY = region.label.position.y < region.centroid?.y ? -10 : 10;
        const eOffsetX = region.label.position.x < region.centroid?.x ? 85 : -85;
        const eOffsetY = region.label.position.y < region.centroid?.y ? 20 : -20;

        const startPoint = region.label.altDrawMode
          ? { x: region.centroid?.x, y: region.centroid?.y + sOffsetY }
          : { x: region.centroid?.x + sOffsetX, y: region.centroid?.y };
        const midPoint = region.label.altDrawMode
          ? { x: region.centroid?.x, y: region.label.position.y }
          : { x: region.label.position.x, y: region.centroid?.y };
        const endPoint = !region.label.altDrawMode
          ? { x: region.label.position.x, y: region.label.position.y + eOffsetY }
          : { x: region.label.position.x + eOffsetX, y: region.label.position.y };

        const lineData = [startPoint, midPoint, endPoint];

        const path = this.layer2
          .append("path")
          .attr("data-interactive", true)
          .attr("d", lineFunction(lineData))
          .attr("stroke-width", 2)
          .attr("stroke", "white")
          .attr("fill", "none");

        const totalLength = path.node().getTotalLength();

        path
          .attr("stroke-dasharray", `${totalLength} ${totalLength}`)
          .attr("stroke-dashoffset", totalLength)
          .transition()
          .delay(400)
          .duration(500)
          .ease(easeSinInOut)
          .attr("stroke-dashoffset", 0);

        this.layer3
          .append("rect")
          .attr("class", "region-label")
          .attr("width", 250)
          .attr("height", 30)
          .attr("x", region.label.position.x - 125)
          .attr("y", region.label.position.y - 15)
          .attr("opacity", 0)
          .transition()
          .duration(500)
          .delay(800)
          .attr("opacity", 1)
          .on("end", () => {
            this.onRegionHighlightEnd(region);
          });

        this.layer3
          .append("text")
          .attr("class", "region-label-text")
          .attr("x", region.label.position.x)
          .attr("y", region.label.position.y + 4)
          .attr("text-anchor", "middle")
          .text(regionKey)
          .attr("opacity", 0)
          .transition()
          .duration(200)
          .delay(1200)
          .attr("opacity", 1);
      }


      this.onRegionHighlightStart(region);
    });
  }

  highlightPathways(pathways: string[]) {
    if (this.isAlreadySelected(pathways)) {
      this.onRegionHighlightStart();
      return;
    }

    this.clearRegionsAndPathways();
    this.clearLabels();
    this.clearCentroids();

    if (!Array.isArray(pathways)) {
      pathways = [pathways];
    }

    pathways.forEach((pathwayKey) => {
      const pathway = this.pathways[pathwayKey];
      if (!pathway) return;

      if (pathway.labels) {
        pathway.labels.forEach((label) => {
          const lineFunction = line<Point>()
            .x((d) => d.x)
            .y((d) => d.y);

          const defaults: Label = {
            position: { x: 0, y: 0 },
            size: { width: 240, height: 32 },
            offset: { x: 0, y: 0 },
          };

          label = { ...defaults, ...label };

          if (Object.values(label.offset).some((v) => v !== 0) || !label.text) {
            this.layer1
              .append("circle")
              .attr("cx", label.position.x)
              .attr("cy", label.position.y)
              .attr("r", 5)
              .attr("fill", "white");
          }

          if (!label.text) return;

          const lineData = [
            label.position,
            {
              x: label.position.x + label.offset.x,
              y: label.position.y + label.offset.y,
            },
          ];

          const path = this.layer2
            .append("path")
            .attr("d", lineFunction(lineData))
            .attr("stroke-width", 2)
            .attr("stroke", "white")
            .attr("fill", "none");

          const totalLength = path?.node()?.getTotalLength();

          path
            .attr("stroke-dasharray", `${totalLength} ${totalLength}`)
            .attr("stroke-dashoffset", totalLength ?? 0)
            .transition()
            .delay(400)
            .duration(500)
            .ease(easeSinInOut)
            .attr("stroke-dashoffset", 0);

          this.layer3
            .append("rect")
            .attr("class", "region-label")
            .attr("width", label.size.width)
            .attr("height", label.size.height)
            .attr("x", label.position.x + label.offset.x - label.size.width / 2)
            .attr("y", label.position.y + label.offset.y - label.size.height / 2)
            .attr("opacity", 0)
            .transition()
            .duration(500)
            .delay(800)
            .attr("opacity", 1);

          this.layer3
            .append("text")
            .attr("class", "region-label-text")
            .attr("x", label.position.x + label.offset.x)
            .attr("y", label.position.y + label.offset.y + 4)
            .attr("text-anchor", "middle")
            .text(label.text)
            .attr("opacity", 0)
            .transition()
            .duration(200)
            .delay(1200)
            .attr("opacity", 1);
        });
      }

      // const path = select(pathway.path);
      // path.attr("stroke", pathway.color).attr("stroke-width", 0).transition().duration(500).attr("stroke-width", pathway.width);

      // if (pathway.glow) {
      //   path.style("filter", "url(#glow)");
      // }

      this.onRegionHighlightStart(pathway);
    });
  }

  onRegionHighlightStart(regionOrPathway: Region | Pathway) { }
  onRegionHighlightEnd(regionOrPathway: Region | Pathway) { }
}


export type { Label, Pathway, Pathways, Region, Regions };

export { InteractiveBrain };

