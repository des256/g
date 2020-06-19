if [ $# != 2 ]
then
    echo "init: deinitialize configuration"
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
elif [ $2 == "specific" ]
then
    if [ ! -d "platforms/specific/$2" ]
    then
        echo "Creating new specific configuration: $2"
        mkdir platforms/specific/$2
    fi
fi
cp Cargo.toml platforms/$1/$2
rm Cargo.toml
rm $1-$2.current
rm -rf target
rm Cargo.lock
