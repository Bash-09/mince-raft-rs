VERSION="1.17.1"
DATA_FILES=("blocks.min.json" "entities.min.json" "block_entities.min.json" "models.min.json")

mkdir assets
for file in ${DATA_FILES[@]}; do
    curl "https://gitlab.bixilon.de/bixilon/pixlyzer-data/-/raw/master/version/${VERSION}/${file}" >> assets/${file}
done

mkdir temp
cd temp
curl https://launcher.mojang.com/v1/objects/8d9b65467c7913fcf6f5b2e729d44a1e00fde150/client.jar >> ${VERSION}.jar
unzip ${VERSION}.jar
mv assets/minecraft/textures ../assets/textures
cd ..
rm -rf temp