<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import type { Snippet } from 'svelte';

  let { children }: { children: Snippet } = $props();
</script>

<div class="flex min-h-screen bg-gradient-to-br from-stone-50 to-amber-50/40 text-stone-800">
  <Sidebar />
  <main class="ml-60 flex-1 p-8 min-h-screen">
    {@render children()}
  </main>
</div>
