<script lang="ts">
  import { page } from '$app/stores';
  import { LayoutDashboard, Network, Monitor, Box, Store, TreePine } from 'lucide-svelte';

  const navItems = [
    { href: '/',           icon: LayoutDashboard, label: 'Dashboard' },
    { href: '/nodes',      icon: Network,          label: 'Nodes'     },
    { href: '/devices',    icon: Monitor,          label: 'Devices'   },
    { href: '/services',   icon: Box,              label: 'Services'  },
    { href: '/catalogue',  icon: Store,            label: 'Catalogue' }
  ];
</script>

<aside class="w-60 h-screen bg-stone-900 border-r border-stone-800 flex flex-col fixed left-0 top-0">
  <div class="px-6 py-5 border-b border-stone-800">
    <div class="flex items-center gap-3">
      <TreePine size={20} class="text-green-400" />
      <div>
        <p class="text-stone-100 font-semibold text-sm leading-none">My Atrium</p>
        <p class="text-stone-500 text-xs mt-1">Canopy</p>
      </div>
    </div>
  </div>

  <nav class="flex-1 px-3 py-4 space-y-0.5">
    {#each navItems as { href, icon: Icon, label }}
      {@const active = $page.url.pathname === href}
      <a
        {href}
        class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-colors
          {active
            ? 'bg-stone-800 text-green-400'
            : 'text-stone-400 hover:text-stone-100 hover:bg-stone-800'}"
      >
        <Icon size={17} />
        {label}
      </a>
    {/each}
  </nav>

  <div class="px-5 py-4 border-t border-stone-800">
    <div class="flex items-center gap-2">
      <div class="w-1.5 h-1.5 rounded-full bg-green-500"></div>
      <span class="text-stone-500 text-xs">All good · 14d uptime</span>
    </div>
  </div>
</aside>
