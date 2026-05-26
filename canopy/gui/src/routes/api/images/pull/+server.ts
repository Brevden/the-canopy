import { json, error } from '@sveltejs/kit';
import { daemonPost } from '$lib/server/daemon';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request }) => {
  const { image } = await request.json();
  if (!image) return error(400, 'Missing field: image');

  try {
    const result = await daemonPost('/images/pull', { image });
    return json(result);
  } catch (e) {
    const message = e instanceof Error ? e.message : 'Unknown error';
    console.error('Pull failed:', message);
    return error(503, 'Daemon unavailable');
  }
};
