# bin/sh
set -o errexit
set -o nounset
set -o pipefail

# Bash Select (Make Menu) https://linuxize.com/post/bash-select/

echo ""
echo "-----------------------------------------"
echo "Select the number of the example to run: "
echo "-----------------------------------------"
echo "1) prepare: Prepare Proton for the examples: create table & load data"
echo "2) query: Query Proton with sample queries"
echo "3) remove: Cleanup Proton and delete streams"
echo "4) quit: Exit"
echo ""
echo "-----------------------------------------"
echo "Make sure Proton is running"
echo "-----------------------------------------"
echo ""

select opt in  prepare query remove quit;
do
  case $opt in

    prepare)
        echo "Selected example: Prepare Proton for the examples"
        command cargo run --example prepare
        break
        ;;

    query)
        echo "Selected example: Query Proton with sample queries"
        command cargo run --example query
        break
        ;;

    remove)
        echo "Selected example: Cleanup Proton and delete streams"
        command cargo run --example remove
        break
        ;;

    quit)
      echo "Exiting!"
      exit 0
      ;;

    *)
      echo "Invalid option $REPLY"
      ;;
  esac
done