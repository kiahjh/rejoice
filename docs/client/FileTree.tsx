import { For, Show, createSignal, type JSX } from "solid-js";

interface FileTreeItem {
  name: string;
  type: "file" | "folder";
  comment?: string;
  children?: FileTreeItem[];
}

interface Props {
  items: FileTreeItem[];
}

export default function FileTree(props: Props): JSX.Element {
  return (
    <div
      class="my-6 p-5 rounded-2xl overflow-x-auto font-mono text-sm"
      style={{ background: "var(--surface-1)", border: "1px solid var(--line)" }}
    >
      <For each={props.items}>
        {(item) => <FileTreeNode item={item} depth={0} />}
      </For>
    </div>
  );
}

function FileTreeNode(props: { item: FileTreeItem; depth: number }): JSX.Element {
  const [expanded, setExpanded] = createSignal(true);
  const isFolder = () => props.item.type === "folder";
  const hasChildren = () => isFolder() && props.item.children && props.item.children.length > 0;

  return (
    <div>
      <div
        class="flex items-center gap-2 py-1.5 px-2 my-0.5 rounded-lg transition-colors cursor-default hover:bg-white/5"
        style={{ "padding-left": `${props.depth * 20 + 8}px` }}
        onClick={() => hasChildren() && setExpanded(!expanded())}
      >
        <span class="flex-shrink-0 w-[18px] h-[18px] flex items-center justify-center">
          <Show when={isFolder()} fallback={<FileIcon name={props.item.name} />}>
            <FolderIcon expanded={expanded()} />
          </Show>
        </span>

        <span
          style={{ color: isFolder() ? "var(--text-bright)" : "var(--text)" }}
          class={isFolder() ? "font-medium" : ""}
        >
          {props.item.name}
        </span>

        <Show when={props.item.comment}>
          <span class="ml-auto pl-4 text-xs italic" style={{ color: "var(--text-ghost)" }}>
            {props.item.comment}
          </span>
        </Show>
      </div>

      <Show when={hasChildren() && expanded()}>
        <For each={props.item.children}>
          {(child) => <FileTreeNode item={child} depth={props.depth + 1} />}
        </For>
      </Show>
    </div>
  );
}

function FolderIcon(props: { expanded: boolean }): JSX.Element {
  return (
    <svg
      width="18"
      height="18"
      viewBox="0 0 24 24"
      fill="none"
      style={{
        color: "var(--ember-bright)",
        transform: props.expanded ? "rotate(0deg)" : "rotate(-90deg)",
        transition: "transform 0.15s ease",
      }}
    >
      <path
        d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
        fill="currentColor"
        opacity="0.2"
      />
      <path
        d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
        fill="none"
      />
    </svg>
  );
}

function FileIcon(props: { name: string }): JSX.Element {
  const ext = props.name.split(".").pop()?.toLowerCase() || "";
  
  const color = 
    ext === "rs" ? "#dea584" :
    ["ts", "tsx", "js", "jsx"].includes(ext) ? "#3178c6" :
    ext === "css" ? "#38bdf8" :
    ext === "toml" ? "#9a8873" :
    ["json", "yaml", "yml"].includes(ext) || props.name.startsWith(".") ? "var(--text-ghost)" :
    "var(--text-ghost)";

  return (
    <svg width="18" height="18" viewBox="0 0 24 24" style={{ color }}>
      <path
        d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"
        fill="currentColor"
        opacity="0.1"
      />
      <path
        d="M14 2H6a2 2 0 00-2 2v16a2 2 0 002 2h12a2 2 0 002-2V8l-6-6z"
        stroke="currentColor"
        stroke-width="1.5"
        fill="none"
      />
      <path d="M14 2v6h6" stroke="currentColor" stroke-width="1.5" fill="none" />
    </svg>
  );
}
