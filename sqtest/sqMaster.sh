source ./sqCommon.sh

sq_master_base=(
  "sq"
  "key"
  "generate"
  "--cannot-sign"
  "--cannot-encrypt"
  "--cannot-authenticate"
)

sq_master_extras=(
  "--output"
  "$masterKeyFile"
  "--rev-cert"
  "$masterRevFile"
)

for userid in "${userIds[@]}"; do
  sq_master_extras+=("--userid")
  sq_master_extras+=("$userid")
done

sq_master_command=(
  "${sq_master_base[@]}"
  "${sq_master_extras[@]}"
)

echo "${sq_master_command[@]}"
"${sq_master_command[@]}"
