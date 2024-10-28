# Template & Guide from
# https://github.com/reckenrode/nix-foundryvtt/blob/main/modules/foundryvtt/default.nix
flake:
{ config
, lib
, pkgs
, ...
}:
let
  cfg = config.services.xinux.bot;
  bot = flake.packages.${pkgs.stdenv.hostPlatform.system}.default;

  genArgs = { cfg }:
  let
    mode = if cfg.webhook.enable then "webhook" else "polling";
    token = cfg.token;
    port = if cfg.webhook.enable then "--port ${cfg.webhook.port}" else "";
    domain =  cfg.webhook.domain or "";
  in
  builtins.trim lib.strings.intersperse " " [mode token domain port];

  caddy = lib.mkIf (cfg.enable && cfg.webhook.enable && cfg.webhook.proxy == "caddy") {
    services.caddy.virtualHosts =
      lib.debug.traceIf (builtins.isNull cfg.webhook.domain) "webhook.domain can't be null, please specicy it properly!" {
      "${cfg.webhook.domain}" = {
        extraConfig = ''
          reverse_proxy 127.0.0.1:${cfg.webhook.port}
        '';
      };
    };
  };

  nginx = lib.mkIf (cfg.enable && cfg.webhook.enable && cfg.webhook.proxy == "nginx") {
    services.nginx.virtualHosts =
    lib.debug.traceIf (builtins.isNull cfg.webhook.domain) "webhook.domain can't be null, please specicy it properly!" {
      "${cfg.webhook.domain}" = {
        addSSL = true;
        enableACME = true;
        locations."/" = {
          proxyPass = "http://127.0.0.1:${cfg.webhook.port}";
          proxyWebsockets = true;
        };
      };
    };
  };
in
{
  options = with lib; {
    services.xinux.bot = {
      enable = lib.mkEnableOption ''
        Xinux Bot: Telegram bot made by Xinux team for Xinux community.
      '';

      webhook = {
        enable = mkEnableOption ''
          Webhook method of deployment
        '';

        domain = mkOption {
          types = with types; nullOr str;
          default = null;
          example = "xinux.uz";
          description = "Domain to use while adding configurations to web proxy server";
        };

        proxy = mkOption {
          types = with types; nullOr enum [
            "nginx"
            "caddy"
          ];
          default = "caddy";
          description = "Proxy reverse software for hosting webhook";
        };

        port = mkOption {
          types = types.int;
          default = 8450;
          description = "Port to use for passing over proxy";
        };
      };

      token = mkOption {
        type = with types; nullOr path;
        default = null;
        description = lib.mdDoc ''
          Path to telegram bot token of Xinux manager.
        '';
      };

      dataDir = mkOption {
        type = types.str;
        default = "/var/lib/xinux/bot ${getMode cfg.webhook.enable} ";
        description = lib.mdDoc ''
          The path where Xinux Bot keeps its config, data, and logs.
        '';
      };

      package = mkOption {
        type = types.package;
        default = bot;
        description = ''
          The Xinux Bot package to use with the service.
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
    warnings = [
      lib.mkIf
        (cfg.webhook.enable && cfg.webhook.url == null)
        ''services.xinux.bot.webhook.url must be set in order to properly generate certificate!''
    ];

    assertions = [ <CODE> ];

    users.users.xinux-bot = {
      description = "Xinux Bot management user";
      isSystemUser = true;
      group = "xinux-bot";
    };

    users.groups.xinux-bot = { };

    systemd.services.xinux-bot = {
      description = "Xinux Bot for managing telegram community";
      documentation = [ "https://xinux.uz/" ];

      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];
      wantedBy = [ "multi-user.target" ];

      serviceConfig = {
        User = "xinux-bot";
        Group = "xinux-bot";
        Restart = "always";
        ExecStart = "${lib.getBin cfg.package}/bin/bot ${genArgs { cfg = cfg; }}";
        StateDirectory = "xinux-bot";
        StateDirectoryMode = "0750";
        # EnvironmentFile = cfg.secret;

        # Hardening
        CapabilityBoundingSet = [
          "AF_NETLINK"
          "AF_INET"
          "AF_INET6"
        ];
        DeviceAllow = [ "/dev/stdin r" ];
        DevicePolicy = "strict";
        IPAddressAllow = "localhost";
        LockPersonality = true;
        # MemoryDenyWriteExecute = true;
        NoNewPrivileges = true;
        PrivateDevices = true;
        PrivateTmp = true;
        PrivateUsers = true;
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectSystem = "strict";
        ReadOnlyPaths = [ "/" ];
        RemoveIPC = true;
        RestrictAddressFamilies = [
          "AF_NETLINK"
          "AF_INET"
          "AF_INET6"
        ];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = "native";
        SystemCallFilter = [
          "@system-service"
          "~@privileged"
          "~@resources"
          "@pkey"
        ];
        UMask = "0027";
      };

      # preStart = ''
      #   installedConfigFile="${config.services.xinux.bot.dataDir}/Config/options.json"
      #   install -d -m750 ${config.services.xinux.bot.dataDir}/Config
      #   rm -f "$installedConfigFile" && install -m640 ${configFile} "$installedConfigFile"
      # '';
    };
  } // nginx // caddy;
}
