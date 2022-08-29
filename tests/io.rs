#[test]
fn chaco() {
    let chaco = "1 0\n\n";
    let gr = ogdf_rs::io::reader::read_chaco(chaco.to_string()).unwrap();
    assert_eq!(Ok(chaco.to_string()), ogdf_rs::io::writer::write_chaco(gr));
}

#[test]
fn graph_ml() {
    let graph_ml = "<?xml version=\"1.0\"?>\n<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns&#10;http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">\n\t<graph id=\"G\" edgedefault=\"directed\">\n\t\t<node id=\"0\" />\n\t\t<node id=\"1\" />\n\t\t<edge id=\"0\" source=\"0\" target=\"1\" />\n\t</graph>\n</graphml>\n";
    let gr = ogdf_rs::io::reader::read_graph_ml(graph_ml.to_string()).unwrap();
    assert_eq!(
        Ok(graph_ml.to_string()),
        ogdf_rs::io::writer::write_graph_ml(gr)
    );
}
