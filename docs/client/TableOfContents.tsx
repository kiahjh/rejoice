import { onMount, onCleanup } from "solid-js";

interface Heading {
  id: string;
  text: string;
  level: number;
}

export default function TableOfContents() {
  onMount(() => {
    const container = document.getElementById("toc-container");
    if (!container) return;

    // Extract headings from the prose article
    const article = document.querySelector("article.prose");
    if (!article) return;

    const elements = article.querySelectorAll("h2, h3");
    const headings: Heading[] = [];

    elements.forEach((el) => {
      // Generate ID if not present
      if (!el.id) {
        el.id =
          el.textContent
            ?.toLowerCase()
            .replace(/[^a-z0-9]+/g, "-")
            .replace(/(^-|-$)/g, "") || "";
      }

      // Add scroll margin for fixed header
      (el as HTMLElement).style.scrollMarginTop = "6rem";

      // Add anchor link functionality to headings
      const heading = el as HTMLElement;
      heading.style.position = "relative";
      heading.classList.add("heading-anchor");

      // Create anchor link button
      const anchor = document.createElement("a");
      anchor.href = `#${el.id}`;
      anchor.className = "heading-anchor-link";
      anchor.innerHTML = `<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path></svg>`;
      anchor.onclick = (e) => {
        e.preventDefault();
        navigator.clipboard.writeText(window.location.origin + window.location.pathname + `#${el.id}`);
        history.pushState(null, "", `#${el.id}`);
        
        // Brief visual feedback
        anchor.style.color = "var(--ember-bright)";
        setTimeout(() => {
          anchor.style.color = "";
        }, 1000);
      };

      heading.appendChild(anchor);

      headings.push({
        id: el.id,
        text: el.textContent || "",
        level: parseInt(el.tagName[1]),
      });
    });

    if (headings.length === 0) {
      container.style.display = "none";
      return;
    }

    const minLevel = Math.min(...headings.map((h) => h.level));

    // Render the TOC with sliding indicator
    container.innerHTML = `
      <h4 class="text-xs font-medium uppercase tracking-widest mb-4" style="color: var(--ink-ghost);">
        On This Page
      </h4>
      <div class="toc-wrapper">
        <div class="toc-indicator"></div>
        <ul class="toc-list">
          ${headings
            .map((h) => {
              const indent = (h.level - minLevel) * 12;
              return `
              <li>
                <a
                  href="#${h.id}"
                  data-id="${h.id}"
                  class="toc-link block py-1.5 text-sm transition-colors duration-200"
                  style="padding-left: ${indent + 12}px;"
                >
                  ${h.text}
                </a>
              </li>
            `;
            })
            .join("")}
        </ul>
      </div>
    `;

    const indicator = container.querySelector(".toc-indicator") as HTMLElement;
    const tocList = container.querySelector(".toc-list") as HTMLElement;

    // Set up intersection observer for active section tracking
    let currentActiveId = "";

    const setActive = (id: string) => {
      if (id === currentActiveId) return;
      currentActiveId = id;

      const links = container.querySelectorAll(".toc-link");
      links.forEach((link) => {
        const linkId = link.getAttribute("data-id");
        const isActive = linkId === id;
        link.classList.toggle("active", isActive);

        // Move the indicator to the active link
        if (isActive && indicator) {
          const linkEl = link as HTMLElement;
          const listTop = tocList.getBoundingClientRect().top;
          const linkTop = linkEl.getBoundingClientRect().top;
          const offset = linkTop - listTop;
          
          indicator.style.transform = `translateY(${offset}px)`;
          indicator.style.height = `${linkEl.offsetHeight}px`;
          indicator.style.opacity = "1";
        }
      });
    };

    const observer = new IntersectionObserver(
      (entries) => {
        // Get all currently visible headings
        const visibleEntries = entries.filter((e) => e.isIntersecting);
        
        if (visibleEntries.length > 0) {
          // Sort by position and take the topmost
          visibleEntries.sort(
            (a, b) => a.boundingClientRect.top - b.boundingClientRect.top
          );
          setActive(visibleEntries[0].target.id);
        }
      },
      {
        rootMargin: "-80px 0px -70% 0px",
        threshold: 0,
      }
    );

    elements.forEach((el) => observer.observe(el));

    // Handle smooth scroll on click
    container.addEventListener("click", (e) => {
      const link = (e.target as HTMLElement).closest("a");
      if (!link) return;

      e.preventDefault();
      const id = link.getAttribute("data-id");
      if (!id) return;

      const el = document.getElementById(id);
      if (el) {
        el.scrollIntoView({ behavior: "smooth" });
        history.pushState(null, "", `#${id}`);
        setActive(id);
      }
    });

    // Initialize indicator position after a brief delay
    setTimeout(() => {
      if (headings.length > 0) {
        setActive(headings[0].id);
      }
    }, 100);

    onCleanup(() => observer.disconnect());
  });

  return null;
}
