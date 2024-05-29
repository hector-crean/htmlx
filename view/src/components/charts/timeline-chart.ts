import { EventDatum } from "@/data/events";
import * as Plot from "@observablehq/plot";

type ChartDimension = {
    width: number, height: number
}
export function timeline(events: Array<EventDatum>, {width, height}: Partial<ChartDimension> = {}) {
  return Plot.plot({
    width,
    height,
    marginTop: 30,
    x: {nice: true, label: null, tickFormat: ""},
    y: {axis: null},
    marks: [
      Plot.ruleX(events, {x: "year", y: "y", markerEnd: "dot", strokeWidth: 2.5}),
      Plot.ruleY([0]),
      Plot.text(events, {x: "year", y: "y", text: "name", lineAnchor: "bottom", dy: -10, lineWidth: 10, fontSize: 12})
    ]
  });
}
