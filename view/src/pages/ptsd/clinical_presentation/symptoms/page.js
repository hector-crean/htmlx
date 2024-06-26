
import { InteractiveBrain } from '@/lib/components/interative-brain';
import tabs from '@/lib/components/tabs';
import pathways from './brain-pathways.json';
import regions from './brain-regions.json';


const CONTAINER_IDS = [
    "arousal-interactive-brain",
    "intrusion-interactive-brain",
    "avoidance-interactive-brain",
    "negative-interactive-brain"
]


const initTabbedBrain = (containerId, regions, pathways) => {
    const containerEl = document.querySelector(`#${containerId}`);

    if (!containerEl) return;

    const svg = containerEl.querySelector('svg[id="interactive-svg"]');
    const descriptionPanel = containerEl.querySelector('p[id="description-panel"]');
    const overviewPanel = containerEl.querySelector('p[id="overview-panel"]');
    const buttons = containerEl.querySelectorAll('button[data-regions]');

    const definitionEls = containerEl.querySelectorAll('[data-definition-id]');



    const getUniqueAttributes = (attribute) => {
        const uniqueAttributes = [];
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
        .reduce((obj, key) => ({ ...obj, [key]: regions[key] }), {});

    const selectedPathways = Object.keys(pathways)
        .filter((key) => featuredPathways.includes(key))
        .reduce((obj, key) => ({ ...obj, [key]: pathways[key] }), {});


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
        const descriptionContainer = button.querySelector('[data-kind="description"]');
        // const overviewContainer = button.querySelector('[data-kind="overview"]');

        // if (descriptionContainer || !overviewContainer) return;

        const description = descriptionContainer.innerHTML;
        descriptionContainer.style.display = "none";



        // const overview = overviewContainer.innerHTML;
        // overviewContainer.style.display = "none";

        button.addEventListener("click", () => {
            buttons.forEach((btn) => btn.classList.remove("active"));
            button.classList.add("active");
            if (descriptionPanel) descriptionPanel.innerHTML = description;
            // if (overviewPanel) overviewPanel.innerHTML = overview;

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




const page = {
    init: () => {
        tabs.init()
        CONTAINER_IDS.forEach(id => initTabbedBrain(id, regions, pathways))
    }
};

export default page;
