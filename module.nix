# Template & Guide from
# https://github.com/reckenrode/nix-foundryvtt/blob/main/modules/foundryvtt/default.nix
flake:
{ config
, lib
, pkgs
, ...
}:
let
  cfg = config.services.xinuxbots;
  xinuxbots = flake.packages.${pkgs.stdenv.hostPlatform.system}.default;
in
{
  options = {
    services.xinuxbots = {
      enable = lib.mkEnableOption ''
        Xinux Bots: Telegram bots made by Xinux team for Xinux community.
      '';

      dataDir = lib.mkOption {
        type = lib.types.str;
        default = "/var/lib/xinuxbots";
        description = lib.mdDoc ''
          The path where Xinux Bots keeps its config, data, and logs.
        '';
      };

      secret = lib.mkOption {
        type = lib.types.nullOr lib.types.path;
        default = null;
        description = ''
          Path to secret key of Xinux Bots.
        '';
      };

      package = lib.mkOption {
        type = lib.types.package;
        default = xinuxbots;
        description = ''
          The Xinux Bots package to use with the service.
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.xinuxbots = {
      description = "Xinux bots management user";
      isSystemUser = true;
      group = "xinuxbots";
    };

    users.groups.xinuxbots = { };

    systemd.services.xinuxbots = {
      description = "Xinux bots deployment";
      documentation = [ "https://xinux.uz/" ];

      after = [ "network-online.target" ];
      wants = [ "network-online.target" ];
      wantedBy = [ "multi-user.target" ];

      environment = {
        LANG = "en_US.UTF-8";
        PATH = cfg.secret;
      };

      serviceConfig = {
        User = "xinuxbots";
        Group = "xinuxbots";
        Restart = "always";
        ExecStart = "${lib.getBin cfg.package}/bin/xinuxmgr";
        StateDirectory = "xinuxbots";
        StateDirectoryMode = "0750";

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
      #   installedConfigFile="${config.services.xinuxbots.dataDir}/Config/options.json"
      #   install -d -m750 ${config.services.xinuxbots.dataDir}/Config
      #   rm -f "$installedConfigFile" && install -m640 ${configFile} "$installedConfigFile"
      # '';
    };
  };
}
