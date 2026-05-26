import http from 'node:http';

// In production this will be /run/canopy/daemon.sock
const SOCKET_PATH = '/tmp/canopy-daemon.sock';

export function daemonPost(path: string, body: unknown): Promise<unknown> {
  const data = JSON.stringify(body);
  return new Promise((resolve, reject) => {
    const req = http.request(
      {
        socketPath: SOCKET_PATH,
        path,
        method: 'POST',
        headers: {
          host: 'localhost',
          'content-type': 'application/json',
          'content-length': Buffer.byteLength(data),
        },
      },
      (res) => {
        let raw = '';
        res.on('data', (chunk: string) => (raw += chunk));
        res.on('end', () => {
          try {
            resolve(JSON.parse(raw));
          } catch {
            reject(new Error(`Bad JSON from daemon: ${raw.slice(0, 200)}`));
          }
        });
      }
    );
    // Pulls can take several minutes for large images
    req.setTimeout(300_000, () => {
      req.destroy(new Error('Pull timed out'));
    });
    req.on('error', reject);
    req.write(data);
    req.end();
  });
}

export function daemonGet(path: string): Promise<unknown> {
  return new Promise((resolve, reject) => {
    const req = http.get(
      { socketPath: SOCKET_PATH, path, headers: { host: 'localhost' } },
      (res) => {
        let raw = '';
        res.on('data', (chunk: string) => (raw += chunk));
        res.on('end', () => {
          try {
            resolve(JSON.parse(raw));
          } catch {
            reject(new Error(`Bad JSON from daemon: ${raw.slice(0, 200)}`));
          }
        });
      }
    );
    req.setTimeout(15_000, () => {
      req.destroy(new Error('Daemon request timed out'));
    });
    req.on('error', reject);
  });
}
