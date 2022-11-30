VERSION="1.16.3"
DATA_FILES=("blocks.min.json" "entities.min.json" "block_entities.min.json" "models.min.json")

mkdir assets
for file in ${DATA_FILES[@]}; do
    curl "https://gitlab.bixilon.de/bixilon/pixlyzer-data/-/raw/master/version/${VERSION}/${file}" >> assets/${file}
done

mkdir temp
cd temp
curl https://launcher.mojang.com/v1/objects/1321521b2caf934f7fc9665aab7e059a7b2bfcdf/client.jar >> ${VERSION}.jar
unzip ${VERSION}.jar
mv assets/minecraft/textures ../assets/textures
cd ..
rm -rf temp
