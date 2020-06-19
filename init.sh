if [ $# != 2 ]
then
    echo "init: initialize configuration"
    echo
    echo "usage: ./init.sh generic <config>"
    echo "or: ./init.sh specific <config>"
    echo
    echo "Generic <config> should be one of: linux, windows, macos, android, ios or web"
    exit 0
fi
if [ $1 == "generic" ]
then
    if [ $2 != "linux" ] && [ $2 != "windows" ] && [ $2 != "macos" ] && [ $2 != "android" ] && [ $2 != "ios" ] && [ $2 == "web" ]
    then
        echo "Generic <config> should be one of: linux, windows, macos, android, ios or web"
        exit 0
    fi
elif [ $1 != "specific" ]
then
    echo "usage: ./init.sh generic <config>"
    echo "or: ./init.sh specific <config>"
    echo
    echo "Generic <config> should be one of: linux, windows, macos, android, ios or web"
    exit 0
fi
cp platforms/generic/$2/Cargo.toml .
touch $1-$2.current
