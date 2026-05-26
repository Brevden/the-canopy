<script lang="ts">
  import { Box, Play, Square, Plus, Loader } from 'lucide-svelte';

  interface ServiceInfo {
    id: string;
    name: string;
    image: string;
    state: string;
    status: string;
  }

  let services = $state<ServiceInfo[]>([]);
  let loading = $state(true);
  let errorMsg = $state('');

  async function load() {
    try {
      const res = await fetch('/api/services');
      if (!res.ok) {
        errorMsg = res.status === 503 ? 'Canopy daemon is not running.' : `Error ${res.status}`;
        return;
      }
      services = await res.json();
    } catch {
      errorMsg = 'Could not reach the Canopy daemon.';
    } finally {
      loading = false;
    }
  }

  load();
</script>

<div class="space-y-6 max-w-4xl">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold text-stone-800">Services</h1>
      <p class="text-stone-500 text-sm mt-1">Containers running on this Atrium</p>
    </div>
    <a
      href="/catalogue"
      class="flex items-center gap-2 px-4 py-2 bg-green-700 hover:bg-green-600 text-white text-sm font-medium rounded-xl transition-colors"
    >
      <Plus size={16} />
      Add service
    </a>
  </div>

  {#if loading}
    <div class="flex items-center gap-2 text-stone-400 text-sm py-8 justify-center">
      <Loader size={16} class="animate-spin" />
      Loading…
    </div>
  {:else if errorMsg}
    <div class="bg-red-50 border border-red-200 rounded-2xl px-4 py-3 text-red-700 text-sm">
      {errorMsg}
    </div>
  {:else if services.length === 0}
    <div class="text-center py-16 text-stone-400">
      <Box size={32} class="mx-auto mb-3 opacity-40" />
      <p class="text-sm">No containers yet.</p>
      <p class="text-xs mt-1"><a href="/catalogue" class="text-green-700 hover:underline">Browse the catalogue</a> to add your first service.</p>
    </div>
  {:else}
    <div class="space-y-3">
      {#each services as service}
        <div class="bg-white border border-stone-200 rounded-2xl p-5 flex items-center justify-between hover:border-stone-300 hover:shadow-sm transition-all">
          <div class="flex items-center gap-4">
            <div class="w-9 h-9 rounded-xl bg-stone-100 flex items-center justify-center">
              <Box size={17} class="text-stone-400" />
            </div>
            <div>
              <p class="text-sm font-medium text-stone-800">{service.name}</p>
              <p class="text-stone-400 text-xs mt-0.5">{service.image} · {service.status}</p>
            </div>
          </div>
          <div class="flex items-center gap-4">
            <div class="flex items-center gap-1.5">
              <div class="w-1.5 h-1.5 rounded-full {service.state === 'running' ? 'bg-green-500' : 'bg-stone-300'}"></div>
              <span class="text-xs capitalize {service.state === 'running' ? 'text-green-700' : 'text-stone-400'}">
                {service.state}
              </span>
            </div>
            <button
              class="w-8 h-8 rounded-lg border flex items-center justify-center transition-colors
                {service.state === 'running'
                  ? 'border-stone-200 text-stone-400 hover:border-red-300 hover:text-red-500'
                  : 'border-stone-200 text-stone-400 hover:border-green-400 hover:text-green-600'}"
              title={service.state === 'running' ? 'Stop' : 'Start'}
            >
              {#if service.state === 'running'}
                <Square size={13} />
              {:else}
                <Play size={13} />
              {/if}
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
