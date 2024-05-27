import { max } from "d3-array";
import { axisBottom, axisLeft } from "d3-axis";
import { scaleBand, scaleLinear } from "d3-scale";
import { select, selection } from "d3-selection";

import selection_interrupt from "../../../node_modules/d3-transition/src/selection/interrupt";
import selection_transition from "../../../node_modules/d3-transition/src/selection/transition";

selection.prototype.interrupt = selection_interrupt;
selection.prototype.transition = selection_transition;

function stringToSlug(str: string): string {
    return str
        .toLowerCase()
        .trim()
        .replace(/\s+/g, '-')
        .replace(/[^\w\-]+/g, '')
        .replace(/\-\-+/g, '-');
}

interface DataItem {
    percent: number;
    shortTitle: string;
    icon?: string;
}

interface Style {
    barWidth?: number;
    textWidth?: number;
    textSize?: number;
    barHeight?: number;
    iconSize?: number;
    barPadding?: number;
    axisSize?: number;
    barColor?: string;
}

interface Text {
    tooltip?: string;
    selected?: (d: DataItem) => string;
}

export default {
    GenerateGraph: function (
        svgID: string,
        tooltipId: string,
        data: DataItem[],
        style: Style = {},
        text: Text = {},
        icons: ((path: string) => string) | null = null,
    ) {
        const toolTipDefaultHTML = text.tooltip || "tooltip";

        const defaultStyle: Required<Style> = {
            barWidth: 320,
            textWidth: 240,
            textSize: 12,
            barHeight: 42,
            iconSize: 42,
            barPadding: 0.2,
            axisSize: 20,
            barColor: "#4682b4",
        };

        style = { ...defaultStyle, ...style };

        const width = style.barWidth + style.textWidth;
        const height =
            (style.barHeight + style.barPadding * 2) * data.length + style.axisSize;

        const y = scaleBand<string>()
            .range([height - style.axisSize, 0])
            .padding(style.barPadding);

        const x = scaleLinear().range([10, style.barWidth]);

        const tooltip = select<HTMLDivElement, unknown>(tooltipId);
        tooltip.html(toolTipDefaultHTML);

        const svg = select<SVGSVGElement, unknown>(svgID)
            .attr("height", height)
            .attr("viewBox", `0 0 ${width} ${height}`)
            .append("g");

        const touchBackgroundLayer = svg.append("g").classed("touchBGLayer", true);
        const graphLayer = svg.append("g").classed("graphLayer", true);
        const overlayLayer = svg.append("g").classed("overlayLayer", true);
        const touchLayer = svg.append("g").classed("touchLayer", true);

        data.forEach((d) => (d.percent === +d.percent));

        x.domain([0, max(data, (d) => d.percent) || 0]);
        y.domain(data.map((d) => d.shortTitle));

        graphLayer
            .selectAll(".bar")
            .data(data)
            .enter()
            .append("path")
            .attr("data-interactive", true)
            .attr("class", "bar")
            .style("fill", style.barColor)
            .attr("d", (d) =>
                RightRoundedRect(
                    style.textWidth,
                    y(d.shortTitle)!,
                    x(0),
                    y.bandwidth(),
                    15,
                ),
            )
            .transition()
            .duration(1500)
            .attr("d", (d) =>
                RightRoundedRect(
                    style.textWidth,
                    y(d.shortTitle)!,
                    x(d.percent),
                    y.bandwidth(),
                    15,
                ),
            );

        overlayLayer
            .selectAll("path")
            .data(data)
            .enter()
            .append("path")
            .attr("data-interactive", true)
            .attr("d", (d) =>
                RightRoundedRect(
                    style.textWidth,
                    y(d.shortTitle)!,
                    style.barWidth,
                    y.bandwidth(),
                    15,
                ),
            )
            .attr("fill", "#FFFFFF")
            .attr("opacity", 0.3);

        const rectos = touchBackgroundLayer
            .selectAll("rect")
            .data(data)
            .enter()
            .append('rect')
            .attr("data-interactive", true)
            .attr("id", (d, i) => `touchBG${stringToSlug(d.shortTitle)}`)
            .attr("class", "bar")
            .style("fill", style.barColor)
            .attr("opacity", 0.1)
            .attr("width", style.textWidth)
            .attr("x", 0)
            .attr("y", (d) => y(d.shortTitle)!)
            .attr("height", y.bandwidth());

        if (icons) {
            touchLayer
                .selectAll(".bar")
                .data(data)
                .enter()
                .append("image")
                .attr("data-interactive", true)
                .attr("xlink:href", (d) => icons(`./${d.icon}`))
                .attr("width", style.iconSize)
                .attr("height", style.iconSize)
                .attr("x", -style.iconSize / 2)
                .attr("y", (d) => y(d.shortTitle)! - style.barPadding * 2)
                .style("fill", "#FFF").style('pointer', 'cursor');
        }

        touchLayer
            .selectAll(".bar")
            .data(data)
            .enter()
            .append("rect")
            .attr("data-interactive", true)
            .attr("class", "bar")
            .attr("opacity", 0)
            .attr("width", style.textWidth + style.barWidth)
            .attr("x", 0)
            .attr("y", (d) => y(d.shortTitle)! - 7)
            .attr("height", y.bandwidth() + 8)
            .on("pointerover", function (event, d) {
                select(this).style("cursor", "pointer");
            })
            .on("pointerdown", function (event, d) {
                rectos.transition()
                    .duration(200)
                    .style('opacity', 0.1);

                rectos.filter(`#touchBG${stringToSlug(d.shortTitle)}`).transition()
                    .duration(200)
                    .style('opacity', 1);

                tooltip
                    .style("left", `${event.pageX - 50}px`)
                    .style("top", `${event.pageY - 70}px`)
                    .style("display", "inline-block")
                    .html(text.selected ? text.selected(d) : "selected");
            });

        const axisHeight = height - style.axisSize;

        graphLayer
            .append("g")
            .attr("transform", `translate(${style.textWidth}, ${axisHeight})`)
            .call(axisBottom(x));

        graphLayer
            .append("g")
            .attr("transform", `translate(${style.textWidth}, 0)`)
            .style("font-size", `${style.textSize}px`)
            .call(axisLeft(y));
    },
};

function RightRoundedRect(x: number, y: number, width: number, height: number, radius: number): string {
    return (
        `M${x},${y}h${(width - radius)}a${radius},${radius} 0 0 1 ${radius},${radius}v${(height - 2 * radius)}a${radius},${radius} 0 0 1 ${-radius},${radius}h${(radius - width)}z`
    );
}
