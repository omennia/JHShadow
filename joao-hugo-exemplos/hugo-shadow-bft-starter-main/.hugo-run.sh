#!/usr/bin/env bash

# Função copiada do stack overflow para dar parse do ficheiro yaml
function parse_yaml {
   local prefix=$2
   local s='[[:space:]]*' w='[a-zA-Z0-9_]*' fs=$(echo @|tr @ '\034')
   sed -ne "s|^\($s\):|\1|" \
        -e "s|^\($s\)\($w\)$s:$s[\"']\(.*\)[\"']$s\$|\1$fs\2$fs\3|p" \
        -e "s|^\($s\)\($w\)$s:$s\(.*\)$s\$|\1$fs\2$fs\3|p"  $1 |
   awk -F$fs '{
      indent = length($1)/2;
      vname[indent] = $2;
      for (i in vname) {if (i > indent) {delete vname[i]}}
      if (length($3) > 0) {
         vn=""; for (i=0; i<indent; i++) {vn=(vn)(vname[i])("_")}
         printf("%s%s%s=\"%s\"\n", "'$prefix'",vn, $2, $3);
      }
   }'
}

# Checking if Shadow is installed and resolving if not
if ! command -v "shadow" &> /dev/null
then
    echo "Shadow was not found in your system.."
    echo "Installing dependencies.."
    ./.install_shadow_deps
    if command -v "shadow" &> /dev/null
    then
      clear
      echo "Shadow installed sucessfully"
      sleep 2
    fi
fi

# # parse_yaml general_config.yaml
eval $(parse_yaml general_config.yaml)


# Checking if we cloned the repo BFT-SmaRT 
if [ ! -d "$BFT_SMART_DIR" ]; then
  echo "BFT-SmaRT directory must be specified in hugo-run.sh."
  echo "You can get it at: https://github.com/bft-smart/library"
  echo "Automatically pulling from github.."
  cd $HOME_DIR
  git clone https://github.com/bft-smart/library.git
  echo " "
  sleep 1
  echo "Successfully cloned from github"
fi


# First we clean the path
if [ -d "$TARGET" ]; then
  echo "  "
  printf '%s\n' "Removing directory ($TARGET)"
  rm -rf "$TARGET"
  echo "  "
fi


# Generating our graph ( Network )
printf 'Now at directory: '
pwd
printf '\n'
g++ -Wall -o mg ./hugo-utilities/make_graph.cpp && ./mg $NUMBER_OF_NODES > network.gml
rm mg

#############################################################################################


# Getting dependencies from library (BFT - smart)
echo "Clearing clashing directories"
if [ -d "config" ]; then
  rm -rf "config"
fi
if [ -d "lib" ]; then
  rm -rf "lib"
fi


# Generating hosts config and system config
g++ -Wall -o ghf ./hugo-utilities/gen_hosts_file.cpp && ./ghf $NUMBER_OF_CLIENTS $NUMBER_OF_REPLICAS
rm ghf
mv ./hosts.config ./library/config/hosts.config
g++ -Wall -o gsc ./hugo-utilities/gen_system_conf.cpp && ./gsc


cd $BFT_SMART_DIR
echo "Now at directory $BFT_SMART_DIR"

pwd
./gradlew installDist
pwd
cp -r ./build/install/library/config $HOME_DIR
cp -r ./build/install/library/lib $HOME_DIR
cd $HOME_DIR
echo " "

# stop point for now
# Testing
cp .syst_conf ./config/system.config
# exit 0
# rm -rf library

# Starting shadow
# shadow --use-memory-manager=false shadow.yaml # Por algum motivo com --use-memory-manager=false crasha.. hum
shadow shadow.yaml
rm -r config
rm -r lib
