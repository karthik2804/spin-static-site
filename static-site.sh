#!/bin/sh

if [ "$#" -ne 1 ]; then
    echo "Error: Wrong number of arguments\n"
    echo "Usage: \n"
    echo "spin static-site <Site Name>"
    exit 1
fi

# Create Temp dir to create spin app
TEMP_DIR=$(mktemp -d)
WORK_DIR=$(pwd)


cd $TEMP_DIR
spin new static-fileserver $1 --value "http-path=/..." --accept-defaults
cd $1
mkdir assets
echo "Creating app"
cp -r $WORK_DIR/* assets/

echo "Creating Deploying"
spin deploy

# Clean up temp dir
rm -r $TEMP_DIR