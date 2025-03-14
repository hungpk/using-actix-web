{ pkgs, lib, config, inputs, ... }:
let
  db_name = "mydb";
  test_db_name = "testdb";
  db_user = "myuser";
  db_user_password = "password";
  db_port = 5432;
in
{
  # https://devenv.sh/basics/
  env = {
    DATABASE_URL = "postgresql://${db_user}:${db_user_password}@localhost:${toString db_port}/${db_name}?schema=public";
    TEST_DATABASE_URL = "postgresql://${db_user}:${db_user_password}@localhost:${toString db_port}/${test_db_name}?schema=public";
  };

  # https://devenv.sh/packages/
  packages = [ 
    pkgs.git
    pkgs.postgresql_15
    pkgs.openssl
  ];
  process.manager.implementation = "process-compose";
  # https://devenv.sh/languages/
  languages.rust.enable = true;
  pre-commit.hooks = {
    clippy.enable = true;
    rustfmt.enable = true;
  };
  # https://devenv.sh/processes/
  #processes.cargo-watch.exec = "cargo watch -x run";

  # https://devenv.sh/services/
  
  services = {
    redis = {
      enable = true;
      port = 6379;
    };
    postgres = {
      enable = true;
      port = 5432;
      listen_addresses = "127.0.0.1";
      initialDatabases = [
        {
          name = "${test_db_name}";
          user = "${db_user}";
          pass = "${db_user_password}";
        }
        { 
          name = "${db_name}"; 
          user = "${db_user}";
          pass = "${db_user_password}";
        }
      ];
    };
  };

  # https://devenv.sh/scripts/
  scripts.hello.exec = ''
    echo "Welcome to my project"
  '';

  enterShell = ''
    export PATH="$PATH:$DEVENV_ROOT/nix_scripts"
  '';

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
    export DATABASE_URL="postgres://${db_user}:${db_user_password}@localhost:5432/${test_db_name}"
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
