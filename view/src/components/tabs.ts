interface Container extends HTMLElement {
	dataset: {
		animation?: string;
	};
}

interface Button extends HTMLElement {
	dataset: {
		active?: string;
	};
}

export default {
	init(): void {
		const reduceToGroups = (
			result: Record<string, Container[]>,
			container: Container,
		): Record<string, Container[]> => {
			const group = container.getAttribute("data-group") || "default";
			if (!(group in result)) {
				result[group] = [];
			}
			result[group].push(container);
			return result;
		};

		const containers = document.getElementsByClassName(
			"toggable-container",
		) as HTMLCollectionOf<Container>;
		const containerGroups = Array.from(containers).reduce(reduceToGroups, {});

		const buttons = document.querySelectorAll(
			"[data-toggable-button]",
		) as NodeListOf<Button>;
		const buttonGroups = Array.from(buttons).reduce(reduceToGroups, {});

		Object.keys(buttonGroups).forEach((group) => {
			const buttonGroup = buttonGroups[group];
			buttonGroup.forEach((button) => {
				button.addEventListener(
					"click",
					(event) => {
						this.selectButton(event.target as Button, buttonGroup);

						const index = parseFloat(button.getAttribute("data-slide") || "0");
						const prev = this.hideActiveContainer(containerGroups[group]);
						this.showContainer(containerGroups[group], index, prev);
						window.dispatchEvent(new Event("resize"));
					},
					false,
				);
			});
		});

		// start with first container visible by default
		Object.keys(buttonGroups).forEach((group) => {
			const buttonGroup = buttonGroups[group];
			if (buttonGroup.length > 0) {
				for (let i = 0; i < containerGroups[group].length; i++) {
					const container = containerGroups[group][i];

					if (container.dataset.animation !== "none") {
						container.style.transition = "0.25s all";
					}

					if (i === 0) continue;
					container.style.display = "none";
					const videos = container.querySelectorAll("video");
					videos.forEach((videoEl) => {
						videoEl.pause();
					});
				}

				buttonGroup[0].click();
			}
		});
	},

	hideActiveContainer(containerGroup: Container[]): number {
		let slide = 0;
		containerGroup.forEach((container) => {
			if (container.style.display !== "none") {
				container.style.display = "none";
				slide = parseFloat(container.getAttribute("data-slide") || "0");
				const videos = container.querySelectorAll("video");
				videos.forEach((videoEl) => {
					videoEl.pause();
				});
			}
		});
		return slide;
	},

	showContainer(
		containerGroup: Container[],
		index: number,
		prev: number | null = null,
		autoplayVideo = false,
	): void {
		containerGroup.forEach((container) => {
			if (parseFloat(container.getAttribute("data-slide") || "0") === index) {
				container.style.removeProperty("display");
				if (container.dataset.animation !== "none") {
					container.style.opacity = "0.25";
					container.style.transform = `translateX(${
						index < prev ? "-" : ""
					}25%)`;
					window.requestAnimationFrame(() => {
						window.getComputedStyle(container).opacity;
						container.style.opacity = "1";
						container.style.transform = "translateX(0)";
					});
				}

				const videos = container.querySelectorAll("video");
				videos.forEach((videoEl, i) => {
					if (autoplayVideo) {
						videoEl.currentTime = 0;
						if (i === 0) {
							videoEl.play();
						}
					}
				});

				const object = container.querySelector("object");
				if (object) {
					object.contentDocument?.location.reload(true);
				}
			}
			container.dispatchEvent(
				new CustomEvent("tabchanged", { detail: { slide_index: index } }),
			);
		});
	},

	selectButton(button: Button, buttonGroup: Button[]): void {
		this.deselectAllButtons(buttonGroup);
		button.classList.add("active");
		button.dataset.active = "true";
	},

	deselectAllButtons(buttonGroup: Button[]): void {
		buttonGroup.forEach((button) => {
			button.classList.remove("active");
			button.dataset.active = "false";
		});
	},
};
