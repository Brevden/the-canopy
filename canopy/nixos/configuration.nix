{ config, pkgs, lib, ... }:

{
  networking.hostName = "canopy";
  time.timeZone = "UTC"; # set per installation

  # ===========================================================
  # Users
  # ===========================================================

  # Unprivileged user for the GUI and Podman containers.
  # The daemon runs as root and communicates with this user
  # via the Unix socket.
  users.users.canopy = {
    isSystemUser = true;
    group = "canopy";
    home = "/var/lib/canopy";
    createHome = true;
  };
  users.groups.canopy = {};

  # ===========================================================
  # WireGuard
  # Interfaces are managed entirely at runtime by the Canopy
  # daemon using wg/ip commands. NixOS only loads the module
  # and provides the tools. This is necessary because:
  #   - mesh IPs are assigned by the coordination server at boot
  #   - peers are added/removed dynamically without nixos-rebuild
  # ===========================================================

  boot.kernelModules = [ "wireguard" ];
  boot.kernel.sysctl."net.ipv4.ip_forward" = 1;

  environment.systemPackages = with pkgs; [
    wireguard-tools
    iproute2
    iptables
  ];

  # ===========================================================
  # Podman
  # Runs containers rootlessly as the canopy user.
  # The daemon manages container lifecycle via the Podman socket.
  # ===========================================================

  virtualisation.podman = {
    enable = true;
    dockerCompat = false;
    defaultNetwork.settings.dns_enabled = true;
    socketActivation = true;
  };

  virtualisation.containers.registries.search = [
    "docker.io"
    "quay.io"
    "ghcr.io"
  ];

  # ===========================================================
  # Unbound — recursive DNS resolver
  # Listens on 127.0.0.1:5335. PiHole forwards clean queries
  # here instead of sending them to an external resolver.
  # ===========================================================

  services.unbound = {
    enable = true;
    settings = {
      server = {
        interface = [ "127.0.0.1@5335" ];
        access-control = [ "127.0.0.1/32 allow" ];
        do-ip4 = true;
        do-ip6 = false;
        do-udp = true;
        do-tcp = true;
        hide-identity = true;
        hide-version = true;
        harden-glue = true;
        harden-dnssec-stripped = true;
        use-caps-for-id = true;
        prefetch = true;
      };
    };
  };

  # ===========================================================
  # PiHole — DNS sinkhole and ad blocker
  # No native NixOS module exists, so it runs as an OCI
  # container via Podman. Uses --network=host so it can bind
  # port 53 and reach Unbound at 127.0.0.1:5335.
  #
  # Note: AdGuard Home (services.adguardhome) is a more
  # NixOS-native alternative if PiHole causes problems.
  # ===========================================================

  virtualisation.oci-containers = {
    backend = "podman";
    containers.pihole = {
      image = "pihole/pihole:latest";
      environment = {
        TZ = "UTC";
        PIHOLE_DNS_ = "127.0.0.1#5335";
        DNSMASQ_LISTENING = "all";
        # WEBPASSWORD should be set via a secrets file, not here.
        # See: https://github.com/Mic92/sops-nix or agenix
        WEBPASSWORD = "";
      };
      volumes = [
        "/var/lib/canopy/pihole/etc:/etc/pihole"
        "/var/lib/canopy/pihole/dnsmasq:/etc/dnsmasq.d"
      ];
      extraOptions = [
        "--network=host"
        "--cap-add=NET_ADMIN"
      ];
    };
  };

  # ===========================================================
  # Caddy — reverse proxy and TLS termination
  # The Caddyfile is written and reloaded by the Canopy daemon
  # as services and nodes are added or removed.
  # A minimal default Caddyfile is created by the daemon on
  # first boot before Caddy starts.
  # ===========================================================

  services.caddy = {
    enable = true;
    configFile = "/etc/canopy/Caddyfile";
  };

  # ===========================================================
  # Canopy Daemon — Rust, runs as root
  # Manages WireGuard interfaces, Podman containers, and Caddy.
  # Exposes a Unix socket that the GUI communicates through.
  # Uses systemd socket activation so the socket is ready
  # before the daemon process is fully started.
  # ===========================================================

  systemd.sockets.canopy-daemon = {
    wantedBy = [ "sockets.target" ];
    socketConfig = {
      ListenStream = "/run/canopy/daemon.sock";
      SocketMode = "0660";
      SocketUser = "root";
      SocketGroup = "canopy";
      DirectoryMode = "0755";
    };
  };

  systemd.services.canopy-daemon = {
    description = "Canopy Daemon";
    after = [ "network.target" "unbound.service" ];
    requires = [ "canopy-daemon.socket" ];
    wantedBy = [ "multi-user.target" ];
    serviceConfig = {
      Type = "notify";
      ExecStart = "/var/lib/canopy/bin/canopy-daemon";
      Restart = "on-failure";
      RestartSec = "5s";
      # Runs as root — required for wg, ip, iptables commands
    };
  };

  # ===========================================================
  # Canopy GUI — Bun + TypeScript
  # Serves the web interface on port 3000 (proxied by Caddy).
  # Communicates with the daemon via the Unix socket.
  # Runs as the unprivileged canopy user.
  # ===========================================================

  systemd.services.canopy-gui = {
    description = "Canopy GUI";
    after = [ "canopy-daemon.service" ];
    wants = [ "canopy-daemon.service" ];
    wantedBy = [ "multi-user.target" ];
    serviceConfig = {
      Type = "simple";
      ExecStart = "${pkgs.bun}/bin/bun run /var/lib/canopy/gui/index.ts";
      Restart = "on-failure";
      RestartSec = "5s";
      User = "canopy";
      Group = "canopy";
    };
  };

  # ===========================================================
  # Firewall
  # ===========================================================

  networking.firewall = {
    enable = true;
    allowedTCPPorts = [ 80 443 ];
    allowedUDPPorts = [
      53    # DNS (PiHole)
      51820 # WireGuard mesh (wg-mesh)
      51821 # WireGuard clients (wg-clients)
    ];
    # Allow DNS queries arriving on WireGuard interfaces
    extraCommands = ''
      iptables -A nixos-fw -i wg-mesh    -p udp --dport 53 -j ACCEPT
      iptables -A nixos-fw -i wg-mesh    -p tcp --dport 53 -j ACCEPT
      iptables -A nixos-fw -i wg-clients -p udp --dport 53 -j ACCEPT
      iptables -A nixos-fw -i wg-clients -p tcp --dport 53 -j ACCEPT
    '';
    extraStopCommands = ''
      iptables -D nixos-fw -i wg-mesh    -p udp --dport 53 -j ACCEPT || true
      iptables -D nixos-fw -i wg-mesh    -p tcp --dport 53 -j ACCEPT || true
      iptables -D nixos-fw -i wg-clients -p udp --dport 53 -j ACCEPT || true
      iptables -D nixos-fw -i wg-clients -p tcp --dport 53 -j ACCEPT || true
    '';
  };

  system.stateVersion = "24.11";
}
