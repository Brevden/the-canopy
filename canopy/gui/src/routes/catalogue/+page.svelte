<script lang="ts">
  import { Search, Star, Package, Loader, Check, AlertCircle } from 'lucide-svelte';

  interface ImageResult {
    name: string;
    description: string;
    stars: number;
    official: boolean;
    registry: string;
  }

  let query = $state('');
  let results = $state<ImageResult[]>([]);
  let loading = $state(false);
  let searched = $state(false);
  let errorMsg = $state('');

  let pulling = $state<Record<string, boolean>>({});
  let pullResult = $state<Record<string, { success: boolean; message: string }>>({});

  async function search() {
    const q = query.trim();
    if (!q) return;

    loading = true;
    searched = true;
    errorMsg = '';
    results = [];

    try {
      const res = await fetch(`/api/images/search?q=${encodeURIComponent(q)}`);
      if (!res.ok) {
        errorMsg = res.status === 503
          ? 'Canopy daemon is not running.'
          : `Search failed (${res.status})`;
        return;
      }
      results = await res.json();
    } catch {
      errorMsg = 'Could not reach the Canopy daemon.';
    } finally {
      loading = false;
    }
  }

  async function pullImage(name: string) {
    pulling = { ...pulling, [name]: true };
    pullResult = { ...pullResult, [name]: undefined! };

    try {
      const res = await fetch('/api/images/pull', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ image: name }),
      });
      if (!res.ok) {
        pullResult = { ...pullResult, [name]: { success: false, message: `HTTP ${res.status}` } };
        return;
      }
      const data = await res.json();
      pullResult = { ...pullResult, [name]: data };
    } catch {
      pullResult = { ...pullResult, [name]: { success: false, message: 'Could not reach daemon' } };
    } finally {
      pulling = { ...pulling, [name]: false };
    }
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Enter') search();
  }

  // Trim the registry prefix for display: docker.io/jellyfin/jellyfin → jellyfin/jellyfin
  function shortName(name: string) {
    const parts = name.split('/');
    return parts.length > 2 ? parts.slice(1).join('/') : parts[parts.length - 1];
  }
</script>

<div class="space-y-6 max-w-4xl">
  <div>
    <h1 class="text-2xl font-semibold text-stone-800">Catalogue</h1>
    <p class="text-stone-500 text-sm mt-1">Search for services to add to your Atrium</p>
  </div>

  <!-- Search bar -->
  <div class="flex gap-3">
    <div class="relative flex-1">
      <Search size={16} class="absolute left-3 top-1/2 -translate-y-1/2 text-stone-400" />
      <input
        type="text"
        placeholder="Search for an image, e.g. jellyfin, navidrome, nextcloud…"
        bind:value={query}
        onkeydown={handleKey}
        class="w-full pl-9 pr-4 py-2.5 bg-white border border-stone-200 rounded-xl text-sm
               text-stone-800 placeholder:text-stone-400 focus:outline-none focus:border-green-500
               focus:ring-1 focus:ring-green-500 transition-colors"
      />
    </div>
    <button
      onclick={search}
      disabled={loading || !query.trim()}
      class="px-5 py-2.5 bg-green-700 hover:bg-green-600 disabled:bg-stone-200
             disabled:text-stone-400 text-white text-sm font-medium rounded-xl
             transition-colors flex items-center gap-2"
    >
      {#if loading}
        <Loader size={15} class="animate-spin" />
        Searching…
      {:else}
        Search
      {/if}
    </button>
  </div>

  <!-- Error -->
  {#if errorMsg}
    <div class="bg-red-50 border border-red-200 rounded-2xl px-4 py-3 text-red-700 text-sm">
      {errorMsg}
    </div>
  {/if}

  <!-- Results -->
  {#if results.length > 0}
    <div>
      <p class="text-stone-400 text-xs mb-3">{results.length} results for "{query}"</p>
      <div class="space-y-2">
        {#each results as image}
          {@const isPulling = pulling[image.name]}
          {@const result = pullResult[image.name]}
          <div class="bg-white border border-stone-200 rounded-2xl p-4 flex items-center
                      justify-between hover:border-stone-300 hover:shadow-sm transition-all">
            <div class="flex items-center gap-4 min-w-0">
              <div class="w-9 h-9 rounded-xl bg-stone-100 flex items-center justify-center shrink-0">
                <Package size={17} class="text-stone-400" />
              </div>
              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <p class="text-sm font-medium text-stone-800 truncate">{shortName(image.name)}</p>
                  {#if image.official}
                    <span class="text-xs bg-green-100 text-green-700 px-1.5 py-0.5 rounded-md shrink-0">
                      Official
                    </span>
                  {/if}
                </div>
                <p class="text-stone-400 text-xs mt-0.5 truncate">
                  {image.description || image.name}
                </p>
              </div>
            </div>
            <div class="flex items-center gap-5 shrink-0 ml-4">
              {#if image.stars > 0}
                <div class="flex items-center gap-1 text-stone-400">
                  <Star size={12} />
                  <span class="text-xs">{image.stars.toLocaleString()}</span>
                </div>
              {/if}
              <span class="text-xs text-stone-400">{image.registry}</span>

              {#if result?.success}
                <span class="flex items-center gap-1.5 text-xs text-green-700 font-medium px-3 py-1.5">
                  <Check size={13} />
                  Added
                </span>
              {:else if result && !result.success}
                <button
                  onclick={() => pullImage(image.name)}
                  title={result.message}
                  class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium border
                         border-red-200 rounded-lg text-red-600 hover:border-red-400 transition-colors"
                >
                  <AlertCircle size={13} />
                  Retry
                </button>
              {:else}
                <button
                  onclick={() => pullImage(image.name)}
                  disabled={isPulling}
                  class="flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium border rounded-lg
                         transition-colors
                         {isPulling
                           ? 'border-stone-200 text-stone-400 cursor-not-allowed'
                           : 'border-stone-200 text-stone-600 hover:border-green-400 hover:text-green-700'}"
                >
                  {#if isPulling}
                    <Loader size={12} class="animate-spin" />
                    Adding…
                  {:else}
                    Add
                  {/if}
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>

  {:else if searched && !loading && !errorMsg}
    <div class="text-center py-12 text-stone-400">
      <Package size={32} class="mx-auto mb-3 opacity-40" />
      <p class="text-sm">No results for "{query}"</p>
    </div>
  {/if}
</div>
