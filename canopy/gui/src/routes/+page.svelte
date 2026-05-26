<script lang="ts">
  import { Network, Monitor, Box, Cpu, HardDrive, Activity, Loader } from 'lucide-svelte';

  interface ServiceInfo {
    id: string;
    name: string;
    image: string;
    state: string;
    status: string;
  }

  interface SystemInfo {
    hostname: string;
    cpu: number;
    memory: { used: number; total: number };
    disk: { used: number; total: number };
  }

  let services = $state<ServiceInfo[]>([]);
  let system = $state<SystemInfo | null>(null);
  let loadingServices = $state(true);
  let loadingSystem = $state(true);

  async function loadData() {
    const [svcRes, sysRes] = await Promise.allSettled([
      fetch('/api/services'),
      fetch('/api/system'),
    ]);

    if (svcRes.status === 'fulfilled' && svcRes.value.ok) {
      services = await svcRes.value.json();
    }
    loadingServices = false;

    if (sysRes.status === 'fulfilled' && sysRes.value.ok) {
      system = await sysRes.value.json();
    }
    loadingSystem = false;
  }

  loadData();

  const runningCount = $derived(services.filter(s => s.state === 'running').length);
  const memPct = $derived(
    system ? Math.round((system.memory.used / system.memory.total) * 100) : 0
  );
  const diskPct = $derived(
    system ? Math.round((system.disk.used / system.disk.total) * 100) : 0
  );
</script>

<div class="space-y-8 max-w-4xl">
  <div>
    <h1 class="text-2xl font-semibold text-stone-800">
      {system?.hostname ? `${system.hostname}` : 'Your Atrium'}
    </h1>
    <p class="text-stone-500 text-sm mt-1">Everything is running smoothly.</p>
  </div>

  <!-- Summary cards -->
  <div class="grid grid-cols-3 gap-4">
    <a href="/nodes" class="bg-white border border-stone-200 rounded-2xl p-5 hover:border-stone-300 hover:shadow-sm transition-all group">
      <div class="flex items-center justify-between mb-4">
        <span class="text-stone-500 text-sm">Connected Nodes</span>
        <Network size={17} class="text-stone-300 group-hover:text-stone-400 transition-colors" />
      </div>
      <p class="text-3xl font-semibold text-stone-800">0</p>
      <p class="text-stone-400 text-xs mt-1">Mesh coming soon</p>
    </a>

    <a href="/devices" class="bg-white border border-stone-200 rounded-2xl p-5 hover:border-stone-300 hover:shadow-sm transition-all group">
      <div class="flex items-center justify-between mb-4">
        <span class="text-stone-500 text-sm">Devices Online</span>
        <Monitor size={17} class="text-stone-300 group-hover:text-stone-400 transition-colors" />
      </div>
      <p class="text-3xl font-semibold text-stone-800">0</p>
      <p class="text-stone-400 text-xs mt-1">Device mgmt coming soon</p>
    </a>

    <a href="/services" class="bg-white border border-stone-200 rounded-2xl p-5 hover:border-stone-300 hover:shadow-sm transition-all group">
      <div class="flex items-center justify-between mb-4">
        <span class="text-stone-500 text-sm">Services Running</span>
        <Box size={17} class="text-stone-300 group-hover:text-stone-400 transition-colors" />
      </div>
      {#if loadingServices}
        <Loader size={20} class="animate-spin text-stone-300 my-1" />
      {:else}
        <p class="text-3xl font-semibold text-stone-800">{runningCount}</p>
        <p class="text-stone-400 text-xs mt-1">{services.length} total</p>
      {/if}
    </a>
  </div>

  <!-- System resources -->
  <div class="bg-white border border-stone-200 rounded-2xl p-5">
    <h2 class="text-sm font-medium text-stone-700 mb-5">System Resources</h2>
    {#if loadingSystem}
      <div class="flex items-center gap-2 text-stone-400 text-sm py-4">
        <Loader size={14} class="animate-spin" />
        Loading…
      </div>
    {:else if system}
      <div class="space-y-4">
        <div>
          <div class="flex items-center justify-between mb-1.5">
            <span class="text-stone-500 text-xs flex items-center gap-1.5"><Cpu size={13} /> CPU</span>
            <span class="text-stone-600 text-xs">{system.cpu}%</span>
          </div>
          <div class="h-1.5 bg-stone-100 rounded-full overflow-hidden">
            <div class="h-full bg-green-600 rounded-full transition-all" style="width: {system.cpu}%"></div>
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between mb-1.5">
            <span class="text-stone-500 text-xs flex items-center gap-1.5"><Activity size={13} /> Memory</span>
            <span class="text-stone-600 text-xs">{system.memory.used} GB / {system.memory.total} GB</span>
          </div>
          <div class="h-1.5 bg-stone-100 rounded-full overflow-hidden">
            <div class="h-full bg-green-600 rounded-full transition-all" style="width: {memPct}%"></div>
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between mb-1.5">
            <span class="text-stone-500 text-xs flex items-center gap-1.5"><HardDrive size={13} /> Storage</span>
            <span class="text-stone-600 text-xs">{system.disk.used} GB / {system.disk.total} GB</span>
          </div>
          <div class="h-1.5 bg-stone-100 rounded-full overflow-hidden">
            <div class="h-full bg-green-600 rounded-full transition-all" style="width: {diskPct}%"></div>
          </div>
        </div>
      </div>
    {:else}
      <p class="text-stone-400 text-sm">Daemon unavailable.</p>
    {/if}
  </div>

  <!-- Services -->
  <div>
    <div class="flex items-center justify-between mb-4">
      <h2 class="text-sm font-medium text-stone-700">Services</h2>
      <a href="/services" class="text-xs text-stone-400 hover:text-stone-600 transition-colors">View all →</a>
    </div>
    {#if loadingServices}
      <div class="flex items-center gap-2 text-stone-400 text-sm">
        <Loader size={14} class="animate-spin" />
        Loading…
      </div>
    {:else if services.length === 0}
      <p class="text-stone-400 text-sm">
        No containers yet. <a href="/catalogue" class="text-green-700 hover:underline">Browse the catalogue</a> to get started.
      </p>
    {:else}
      <div class="grid grid-cols-2 gap-3">
        {#each services.slice(0, 6) as service}
          <div class="bg-white border border-stone-200 rounded-2xl p-4 flex items-center justify-between hover:border-stone-300 hover:shadow-sm transition-all">
            <div>
              <p class="text-sm font-medium text-stone-800">{service.name}</p>
              <p class="text-stone-400 text-xs mt-0.5">{service.image}</p>
            </div>
            <div class="flex items-center gap-1.5">
              <div class="w-1.5 h-1.5 rounded-full {service.state === 'running' ? 'bg-green-500' : 'bg-stone-300'}"></div>
              <span class="text-xs capitalize {service.state === 'running' ? 'text-green-700' : 'text-stone-400'}">
                {service.state}
              </span>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>
