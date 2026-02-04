import { createSignal, createEffect, onCleanup, onMount } from "solid-js";

interface LogViewerProps {
  projectId: string;
  deploymentId: string;
  initialLogs: string | null;
  initialStatus: string;
  initialFinished: boolean;
}

export default function LogViewer(props: LogViewerProps) {
  const [logs, setLogs] = createSignal(props.initialLogs || "");
  const [status, setStatus] = createSignal(props.initialStatus);
  const [finished, setFinished] = createSignal(props.initialFinished);
  const [userScrolledUp, setUserScrolledUp] = createSignal(false);

  let containerRef: HTMLPreElement | undefined;
  let pollInterval: number | undefined;

  // Check if user has scrolled up from bottom
  const handleScroll = () => {
    if (!containerRef) return;
    const { scrollTop, scrollHeight, clientHeight } = containerRef;
    // Consider "at bottom" if within 50px of the bottom
    const atBottom = scrollHeight - scrollTop - clientHeight < 50;
    setUserScrolledUp(!atBottom);
  };

  // Scroll to bottom (only if user hasn't scrolled up)
  const scrollToBottom = () => {
    if (!containerRef || userScrolledUp()) return;
    containerRef.scrollTop = containerRef.scrollHeight;
  };

  // Fetch latest logs
  const fetchLogs = async () => {
    try {
      const response = await fetch(
        `/projects/${props.projectId}/deployments/${props.deploymentId}/logs`
      );
      if (!response.ok) return;

      const data = await response.json();
      
      const wasInProgress = !finished();
      
      if (data.logs !== null) {
        setLogs(data.logs);
      }
      setStatus(data.status);
      setFinished(data.finished);

      // Scroll to bottom after updating logs
      requestAnimationFrame(scrollToBottom);

      // Stop polling and refresh page if deployment just finished
      if (data.finished && pollInterval) {
        clearInterval(pollInterval);
        pollInterval = undefined;
        
        // If deployment just finished (was in progress before), refresh the page
        // after a short delay so user can see the final status
        if (wasInProgress) {
          setTimeout(() => {
            window.location.reload();
          }, 1500);
        }
      }
    } catch (error) {
      console.error("Failed to fetch logs:", error);
    }
  };

  // Start polling on mount
  onMount(() => {
    // Initial scroll to bottom
    requestAnimationFrame(() => {
      if (containerRef) {
        containerRef.scrollTop = containerRef.scrollHeight;
      }
    });

    // Only poll if not finished
    if (!props.initialFinished) {
      pollInterval = setInterval(fetchLogs, 1500) as unknown as number;
    }
  });

  // Cleanup on unmount
  onCleanup(() => {
    if (pollInterval) {
      clearInterval(pollInterval);
    }
  });

  // Status badge styling
  const statusBadge = () => {
    const s = status();
    switch (s) {
      case "pending":
        return (
          <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-stone-700 text-stone-300">
            Pending
          </span>
        );
      case "building":
      case "deploying":
        return (
          <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-amber-900/50 text-amber-200">
            <span class="mr-1.5 h-1.5 w-1.5 rounded-full bg-amber-400 animate-pulse" />
            {s === "building" ? "Building" : "Deploying"}
          </span>
        );
      case "success":
        return (
          <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-green-900/50 text-green-200">
            Live
          </span>
        );
      case "failed":
        return (
          <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-red-900/50 text-red-200">
            Failed
          </span>
        );
      default:
        return (
          <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-stone-700 text-stone-300">
            {s}
          </span>
        );
    }
  };

  return (
    <div class="rounded-lg border border-stone-800 bg-stone-900 overflow-hidden">
      <div class="flex items-center justify-between px-4 py-3 border-b border-stone-800">
        <h2 class="text-sm font-medium text-stone-300">Build logs</h2>
        <div class="flex items-center gap-3">
          {!finished() && (
            <span class="text-xs text-stone-500">Auto-updating...</span>
          )}
          {statusBadge()}
        </div>
      </div>
      <pre
        ref={containerRef}
        onScroll={handleScroll}
        class="text-xs text-stone-400 whitespace-pre-wrap font-mono bg-stone-950 p-4 overflow-x-auto h-96 overflow-y-auto"
      >
        {logs() || (
          <span class="text-stone-500">
            {status() === "pending"
              ? "Waiting to start..."
              : status() === "building" || status() === "deploying"
              ? "Build starting..."
              : "No logs available"}
          </span>
        )}
      </pre>
      {userScrolledUp() && !finished() && (
        <div class="absolute bottom-4 right-4">
          <button
            onClick={() => {
              setUserScrolledUp(false);
              if (containerRef) {
                containerRef.scrollTop = containerRef.scrollHeight;
              }
            }}
            class="px-3 py-1.5 text-xs font-medium bg-stone-800 text-stone-300 rounded-lg border border-stone-700 hover:bg-stone-700 transition-colors shadow-lg"
          >
            Scroll to bottom
          </button>
        </div>
      )}
    </div>
  );
}
