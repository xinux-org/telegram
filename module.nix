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
in
{
  options = {
    services.xinux.bot = {
      enable = lib.mkEnableOption ''
        Xinux Bot: Telegram bot made by Xinux team for Xinux community.
      '';

      dataDir = lib.mkOption {
        type = lib.types.str;
        default = "/var/lib/xinux/bot";
        description = lib.mdDoc ''
          The path where Xinux Bot keeps its config, data, and logs.
        '';
      };

      secret = lib.mkOption {
        type = lib.types.nullOr lib.types.path;
        default = null;
        description = ''
          Path to secret key of Xinux Bot.
        '';
      };

      package = lib.mkOption {
        type = lib.types.package;
        default = bot;
        description = ''
          The Xinux Bot package to use with the service.
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
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
        ExecStart = "${lib.getBin cfg.package}/bin/bot";
        StateDirectory = "xinux-bot";
        StateDirectoryMode = "0750";
        EnvironmentFile = cfg.secret;

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
  };
}
