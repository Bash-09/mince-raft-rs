
mod generators;

fn main() {

    generators::entities::export_entities("data/1.17.1/entities.json").unwrap();
    // generators::blocks::export_blocks("data/1.17.1/blocks.json").unwrap();

}
