<script lang="ts">
  import { Monitor, Plus } from 'lucide-svelte';
</script>

<div class="space-y-6 max-w-4xl">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold text-stone-800">Devices</h1>
      <p class="text-stone-500 text-sm mt-1">Your personal devices connected to this Atrium</p>
    </div>
    <button class="flex items-center gap-2 px-4 py-2 bg-green-700 hover:bg-green-600 text-white text-sm font-medium rounded-xl transition-colors">
      <Plus size={16} />
      Add device
    </button>
  </div>

  <div class="text-center py-20 text-stone-400">
    <Monitor size={36} class="mx-auto mb-4 opacity-30" />
    <p class="text-sm font-medium text-stone-500">No devices registered</p>
    <p class="text-xs mt-1 max-w-xs mx-auto">Device management is coming soon. You'll be able to connect your phone, laptop, and more.</p>
  </div>
</div>
