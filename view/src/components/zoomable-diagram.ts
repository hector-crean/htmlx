import { mean } from "d3-array";
import { pointers, select } from "d3-selection";
import { zoom, zoomIdentity, zoomTransform } from "d3-zoom";



const zoomableDiagram = (containerEl: HTMLDivElement) => {




    const {width, height} = containerEl.getBoundingClientRect();

    const svg = select(containerEl)
            .append('svg')

    svg.attr('width', '100%')
            .attr('height', '100%')
            .attr('viewport', `${0} ${0} ${width} ${height}`).attr('data-interactive', true)



    const gx = svg.append("g");
    const gy = svg.append("g")


    // z holds a copy of the previous transform, so we can track its changes
    let z = zoomIdentity;

     // set up the ancillary zooms and an accessor for their transforms
    const zoomX = zoom().scaleExtent([0.1, 10]);
    const zoomY = zoom().scaleExtent([0.2, 5]);
    const tx = () => zoomTransform(gx.node());
    const ty = () => zoomTransform(gy.node());
    gx.call(zoomX).attr("pointer-events", "none");
    gy.call(zoomY).attr("pointer-events", "none");

    const zoomFn = zoom().on("zoom", function(e) {
        const t = e.transform;
        const k = t.k / z.k;
        const point = center(e, this);
    
        // is it on an axis? is the shift key pressed?
        const doX = point[0] > x.range()[0];
        const doY = point[1] < y.range()[0];
        const shift = e.sourceEvent?.shiftKey;
    
        if (k === 1) {
          // pure translation?
          doX && gx.call(zoomX.translateBy, (t.x - z.x) / tx().k, 0);
          doY && gy.call(zoomY.translateBy, 0, (t.y - z.y) / ty().k);
        } else {
          // if not, we're zooming on a fixed point
          doX && gx.call(zoomX.scaleBy, shift ? 1 / k : k, point);
          doY && gy.call(zoomY.scaleBy, k, point);
        }
    
        z = t;
    
        redraw();
      });

      function center(event: Event, target: Element) {
        if (event.sourceEvent) {
          const p = pointers(event, target);
          return [mean(p, d => d[0]), mean(p, d => d[1])];
        }
        return [width / 2, height / 2];
      }


    const redraw = () => {

        // const x = scaleLinear()
        // .domain(extent(data, d => d[0]))
        // .range([30, width - 10])
        // .nice()
    
        // const y = scaleLinear()
        // .domain(extent(data, d => d[1]))
        // .range([height - 20, 10])
        // .nice()

        // const xr = tx().rescaleX(x);
        // const yr = ty().rescaleY(y);
    
    }



    return svg
    .call(zoom)
    // .call(zoom.transform, zoomIdentity.scale(0.8))
    .node();



}