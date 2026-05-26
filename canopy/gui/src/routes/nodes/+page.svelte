<script lang="ts">
  import { Network, Plus } from 'lucide-svelte';
</script>

<div class="space-y-6 max-w-4xl">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold text-stone-800">Nodes</h1>
      <p class="text-stone-500 text-sm mt-1">Other Atrium installations connected to your mesh</p>
    </div>
    <button class="flex items-center gap-2 px-4 py-2 bg-green-700 hover:bg-green-600 text-white text-sm font-medium rounded-xl transition-colors">
      <Plus size={16} />
      Add node
    </button>
  </div>

  <div class="text-center py-20 text-stone-400">
    <Network size={36} class="mx-auto mb-4 opacity-30" />
    <p class="text-sm font-medium text-stone-500">No nodes connected</p>
    <p class="text-xs mt-1 max-w-xs mx-auto">Mesh networking is coming soon. You'll be able to connect your Atrium to friends and family here.</p>
  </div>
</div>
