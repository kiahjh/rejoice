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
  const [expanded, setExpanded] = createSignal(true);

  let containerRef: HTMLPreElement | undefined;
  let pollInterval: number | undefined;

  // Check if user has scrolled up from bottom
  const handleScroll = () => {
    if (!containerRef) return;
    const { scrollTop, scrollHeight, clientHeight } = containerRef;
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

      requestAnimationFrame(scrollToBottom);

      if (data.finished && pollInterval) {
        clearInterval(pollInterval);
        pollInterval = undefined;
        
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
    requestAnimationFrame(() => {
      if (containerRef) {
        containerRef.scrollTop = containerRef.scrollHeight;
      }
    });

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
    const baseClasses = "inline-flex items-center gap-1.5 px-2 py-0.5 rounded-md text-xs font-medium border";
    
    switch (s) {
      case "pending":
        return (
          <span class={`${baseClasses} bg-[var(--bg-surface)] text-[var(--text-muted)] border-[var(--border-default)]`}>
            <span class="w-1.5 h-1.5 rounded-full bg-[var(--text-faint)]" />
            Pending
          </span>
        );
      case "building":
      case "deploying":
        return (
          <span class={`${baseClasses} bg-amber-500/10 text-amber-400 border-amber-500/20`}>
            <span class="w-1.5 h-1.5 rounded-full bg-amber-400 animate-pulse" />
            {s === "building" ? "Building" : "Deploying"}
          </span>
        );
      case "success":
        return (
          <span class={`${baseClasses} bg-emerald-500/10 text-emerald-400 border-emerald-500/20`}>
            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400" />
            Complete
          </span>
        );
      case "failed":
        return (
          <span class={`${baseClasses} bg-red-500/10 text-red-400 border-red-500/20`}>
            <span class="w-1.5 h-1.5 rounded-full bg-red-400" />
            Failed
          </span>
        );
      default:
        return (
          <span class={`${baseClasses} bg-[var(--bg-surface)] text-[var(--text-muted)] border-[var(--border-default)]`}>
            {s}
          </span>
        );
    }
  };

  return (
    <div class="rounded-xl border border-[var(--border-subtle)] bg-[var(--bg-elevated)] overflow-hidden shadow-sm">
      {/* Header */}
      <div 
        class="flex items-center justify-between px-4 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-surface)] cursor-pointer hover:bg-[var(--bg-hover)] transition-colors"
        onClick={() => setExpanded(!expanded())}
      >
        <div class="flex items-center gap-3">
          {/* Terminal icon */}
          <div class="w-6 h-6 rounded-md bg-[var(--bg-base)] flex items-center justify-center text-[var(--text-faint)]">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="4 17 10 11 4 5" />
              <line x1="12" y1="19" x2="20" y2="19" />
            </svg>
          </div>
          <h2 class="text-sm font-medium text-[var(--text-primary)]">Build logs</h2>
        </div>
        <div class="flex items-center gap-3">
          {!finished() && (
            <span class="flex items-center gap-1.5 text-xs text-[var(--text-faint)]">
              <span class="w-1 h-1 rounded-full bg-amber-400 animate-pulse" />
              Live
            </span>
          )}
          {statusBadge()}
          {/* Expand/collapse icon */}
          <span class={`text-[var(--text-faint)] transition-transform duration-200 ${expanded() ? '' : '-rotate-90'}`}>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="6 9 12 15 18 9" />
            </svg>
          </span>
        </div>
      </div>
      
      {/* Log content */}
      {expanded() && (
        <div class="relative">
          <pre
            ref={containerRef}
            onScroll={handleScroll}
            class="text-xs text-[var(--text-muted)] whitespace-pre-wrap font-mono bg-[var(--bg-deepest)] p-4 h-[400px] overflow-auto leading-relaxed"
          >
            {logs() || (
              <span class="text-[var(--text-faint)]">
                {status() === "pending"
                  ? "Waiting to start..."
                  : status() === "building" || status() === "deploying"
                  ? "Build starting..."
                  : "No logs available"}
              </span>
            )}
          </pre>
          
          {/* Scroll to bottom button */}
          {userScrolledUp() && !finished() && (
            <div class="absolute bottom-4 right-4">
              <button
                onClick={() => {
                  setUserScrolledUp(false);
                  if (containerRef) {
                    containerRef.scrollTop = containerRef.scrollHeight;
                  }
                }}
                class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium bg-[var(--bg-surface)] text-[var(--text-secondary)] rounded-lg border border-[var(--border-default)] hover:bg-[var(--bg-hover)] hover:border-[var(--border-strong)] transition-all shadow-lg"
              >
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="12" y1="5" x2="12" y2="19" />
                  <polyline points="19 12 12 19 5 12" />
                </svg>
                Scroll to bottom
              </button>
            </div>
          )}
          
          {/* Gradient fade at top */}
          <div class="absolute top-0 left-0 right-0 h-6 bg-gradient-to-b from-[var(--bg-deepest)] to-transparent pointer-events-none" />
        </div>
      )}
    </div>
  );
}
