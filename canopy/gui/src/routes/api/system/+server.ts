import { json, error } from '@sveltejs/kit';
import { daemonGet } from '$lib/server/daemon';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async () => {
  try {
    const result = await daemonGet('/system');
    return json(result);
  } catch (e) {
    const message = e instanceof Error ? e.message : 'Unknown error';
    console.error('System info failed:', message);
    return error(503, 'Daemon unavailable');
  }
};
