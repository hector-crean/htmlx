
import { InteractiveBrain, Pathways, Regions } from '@/components/interative-brain';
import pathways from '@/components/interative-brain/brain-pathways.json';
import regions from '@/components/interative-brain/brain-regions.json';
import tabs from '@/components/tabs';
import Swiper from 'swiper/bundle';










const CONTAINER_IDS = [
  "arousal-interactive-brain",
  "intrusion-interactive-brain",
  "avoidance-interactive-brain",
  "negative-interactive-brain"
]


const initTabbedBrain = (containerId: string, regions: Regions, pathways: Pathways) => {
  const containerEl = document.querySelector<HTMLElement>(`#${containerId}`);

  if (!containerEl) return;

  const svg = containerEl.querySelector<SVGElement>('svg[id="interactive-svg"]');
  const descriptionPanel = containerEl.querySelector<HTMLParagraphElement>('p[id="description-panel"]');
  const overviewPanel = containerEl.querySelector<HTMLParagraphElement>('p[id="overview-panel"]');
  const buttons = containerEl.querySelectorAll<HTMLButtonElement>('button[data-regions]');

  const definitionEls = containerEl.querySelectorAll('[data-definition-id]');



  const getUniqueAttributes = (attribute: string): string[] => {
    const uniqueAttributes: string[] = [];
    buttons.forEach((button) => {
      const attributes = (button.getAttribute(attribute) || "").split(" ");
      attributes.forEach((attr) => {
        if (!uniqueAttributes.includes(attr)) {
          uniqueAttributes.push(attr);
        }
      });
    });
    return uniqueAttributes;
  };

  const featuredRegions = getUniqueAttributes("data-regions");
  const featuredPathways = getUniqueAttributes("data-pathways");

  const selectedRegions = Object.keys(regions)
    .filter((key) => featuredRegions.includes(key))
    .reduce<Regions>((obj, key) => ({ ...obj, [key]: regions[key] }), {});

  const selectedPathways = Object.keys(pathways)
    .filter((key) => featuredPathways.includes(key))
    .reduce<Pathways>((obj, key) => ({ ...obj, [key]: pathways[key] }), {});


  if (!svg) return;

  const brain = new InteractiveBrain(svg, { regions: selectedRegions, pathways: selectedPathways, interactive: false });


  definitionEls.forEach(definitionEl => {
    const defStr = definitionEl.getAttribute('data-definition-id')
    if (defStr) {
      if (!featuredRegions.includes(defStr)) {
        definitionEl.classList.add('hidden')
      }
      definitionEl.addEventListener('click', (e) => {
        brain.highlightRegions([defStr])
      })

    }

  })

  buttons.forEach((button) => {
    const descriptionContainer = button.querySelector<HTMLElement>('[data-kind="description"]');
    const overviewContainer = button.querySelector<HTMLElement>('[data-kind="overview"]');

    if (!descriptionContainer || !overviewContainer) return;

    const description = descriptionContainer.innerHTML;
    descriptionContainer.style.display = "none";

    const overview = overviewContainer.innerHTML;
    overviewContainer.style.display = "none";

    button.addEventListener("click", () => {
      buttons.forEach((btn) => btn.classList.remove("active"));
      button.classList.add("active");
      if (descriptionPanel) descriptionPanel.innerHTML = description;
      if (overviewPanel) overviewPanel.innerHTML = overview;

      const regions = button.getAttribute("data-regions");
      if (regions) {
        const regioArr = regions.split(" ");
        brain.highlightRegions(regioArr);
      }

      const pathways = button.getAttribute("data-pathways");
      if (pathways) {
        const pathwayArr = pathways.split(" ")
        brain.highlightPathways(pathwayArr);
      }
    });
  });
};




export default {
  init: () => {
    // init Swiper:
    const swiper = new Swiper('.swiper', {
      // configure Swiper to use modules
      pagination: {
        el: ".swiper-pagination",
      },
      navigation: {
        nextEl: ".swiper-button-next",
        prevEl: ".swiper-button-prev",
      },
    });
    tabs.init()
    CONTAINER_IDS.forEach(id => initTabbedBrain(id, regions, pathways))


  }
}
