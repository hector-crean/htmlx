import { appEl } from "./main";
export type Routes = {
  [key: string]: {
    title: string;
    content?: Promise<string>;
    scripts?: (() => void)[];
    condition?: () => boolean;
    fallback?: string;
  };
};

let routes: Routes = {};

/**
 * Sets the routes; this function must be called before using the router.
 * @param newRoutes - The new routes.
 */
export const setRoutes = (newRoutes: Routes) => {
  routes = newRoutes;
};

/**
 * Gets the content of a file from the public folder.
 * @param location - The location of the file in the public folder.
 * @returns Promise<string> - The content of the file.
 */
export const getFile = async (location: string): Promise<string> => {
  const response = await fetch(location);
  return response.text();
};

/**
 * Renders the given route.
 * @param route - The route to render.
 */
const render = async (route: string) => {
  const validRoute = routes[route];
  if (!appEl) return;


  if (!validRoute) {
    appEl.innerHTML = "404";
    return;
  }

  if (validRoute.condition && !validRoute.condition()) {
    if (validRoute.fallback) {
      goTo(validRoute.fallback);
    } else {
      window.history.pushState({}, "", "/");
      render(window.location.pathname || "/");
    }
    return;
  }

  document.title = validRoute.title;

  if (validRoute.content) {
    try {
      const content = await validRoute.content;
      appEl.innerHTML = content;
    } catch (error) {
      console.error("Error loading content:", error);
      appEl.innerHTML = "Error loading content";
    }
  } else {
    appEl.innerHTML = "";
  }

  if (validRoute.scripts) {
    validRoute.scripts.forEach((script) => {
      script();
    });
  }
};

/**
 * Sets the route to go to.
 * @param route - The route to navigate to.
 */
export const goTo = (route: string) => {
  history.pushState({}, "", route);
  render(route);
};

/**
 * Gets the value of a parameter from the URL.
 * @param name - The name of the parameter.
 * @returns The value of the parameter.
 */
export const getParam = (name: string): string | null => {
  return new URLSearchParams(window.location.search).get(name);
};

window.onpopstate = () => {
  render(window.location.pathname || "/");
};

/**
 * Initializes the router.
 */
const router = () => {
  render(window.location.pathname || "/"); // Render the initial route
  window.onpopstate = () => {
    render(window.location.pathname || "/");
  };
};

export default router;
