import { json, error } from '@sveltejs/kit';
import { daemonGet } from '$lib/server/daemon';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ url }) => {
  const q = url.searchParams.get('q')?.trim();
  if (!q) return error(400, 'Missing query parameter: q');

  try {
    const results = await daemonGet(`/images/search?q=${encodeURIComponent(q)}`);
    return json(results);
  } catch (e) {
    const message = e instanceof Error ? e.message : 'Unknown error';
    console.error('Daemon unavailable:', message);
    return error(503, 'Daemon unavailable');
  }
};
