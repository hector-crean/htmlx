import { InteractiveBrain } from '../../../components/interative-brain/index';

import pathways from '../../../components/interative-brain/brain-pathways.json';
import regions from '../../../components/interative-brain/brain-regions.json';

const FIGURE_ID = "inbalanced-neurotransmitter-interactive-brain";
const SELECTOR = `figure[data-id="${FIGURE_ID}"]`;

const init = () => {

    const figureEl = document

    const svg = figureEl?.querySelector('svg[id="interactive-svg"]');
  
    const infoPanel = figureEl.querySelector("figcaption[id=info-panel]");
  
    const buttons = figureEl.querySelectorAll("#symptom-btn");

  // go through all the brain regions, keep only the ones that are featured in this section
  const featuredRegions: string[] = [];
  for (let i = 0; i < buttons.length; i++) {
    const button = buttons[i];
    const regions = (button.getAttribute("data-regions") || "").split(" ");
    regions.forEach((region) => {
      if (!featuredRegions.includes(region)) {
        featuredRegions.push(region);
      }
    });
  }

  const featuredPathways: string[] = [];
  for (let i = 0; i < buttons.length; i++) {
    const button = buttons[i];
    const pathways = (button.getAttribute("data-pathways") || "").split(" ");
    pathways.forEach((pathway) => {
      if (!featuredPathways.includes(pathway)) {
        featuredPathways.push(pathway);
      }
    });
  }

  const selectedRegions = Object.keys(regions)
    .filter((key) => featuredRegions.some((r) => r === key))
    .reduce((obj, key) => ({ ...obj, [key]: regions[key] }), {});

  const selectedPathways = Object.keys(pathways)
    .filter((key) => featuredPathways.some((r) => r === key))
    .reduce((obj, key) => ({ ...obj, [key]: pathways[key] }), {});

  const brain = new InteractiveBrain(svg, { regions: selectedRegions, selectedPathways, interactive: false });

  for (let i = 0; i < buttons.length; i++) {
    const button = buttons[i];

    const descriptionHolder = button.querySelector(".info");
    descriptionHolder.style.display = "none";
    const description = descriptionHolder.innerHTML;

    button.addEventListener("click", (e) => {
      for (const b of buttons) {
        b.classList.remove("active");
      }
      button.classList.add("active");

      const regions = button.getAttribute("data-regions");
      if (regions) {
        brain.highlightRegions(regions.split(" "));
      }

      const pathways = button.getAttribute("data-pathways");
      if (pathways) {
        brain.highlightPathways(pathways.split(" "));
      }

      infoPanel.innerHTML = description;
    });
  }

  brain.onRegionHighlightEnd = () => {
    infoPanel.scrollIntoView({ block: "end", behavior: "smooth" });
  };



}


export default init;