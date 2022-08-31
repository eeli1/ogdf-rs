#[test]
fn chaco() {
    let code = "1 0\n\n";
    let gr = ogdf_rs::io::reader::read_chaco(code.to_string()).unwrap();
    assert_eq!(Ok(code.to_string()), ogdf_rs::io::writer::write_chaco(gr));
}

#[test]
fn dl() {
    let code = "DL N = 1\nFORMAT = edgelist1\nDATA:\n";
    let gr = ogdf_rs::io::reader::read_dl(code.to_string()).unwrap();
    assert_eq!(Ok(code.to_string()), ogdf_rs::io::writer::write_dl(gr));
}

#[test]
fn dot() {
    let code = "digraph G {\n\t0\n\t1\n\n}\n";
    let gr = ogdf_rs::io::reader::read_dot(code.to_string()).unwrap();
    assert_eq!(Ok(code.to_string()), ogdf_rs::io::writer::write_dot(gr));
}

#[test]
fn gdf() {
    let code = "nodedef>name\nn0\nedgedef>node1,node2\n";
    let gr = ogdf_rs::io::reader::read_gdf(code.to_string()).unwrap();
    assert_eq!(Ok(code.to_string()), ogdf_rs::io::writer::write_gdf(gr));
}

#[test]
fn gexf() {
    let code = "<?xml version=\"1.0\"?>\n<gexf version=\"1.2\" xmlns=\"http://www.gexf.net/1.2draft\">\n\t<graph mode=\"static\" defaultedgetype=\"directed\">\n\t\t<nodes>\n\t\t\t<node id=\"0\" />\n\t\t</nodes>\n\t\t<edges />\n\t</graph>\n</gexf>\n";
    let gr = ogdf_rs::io::reader::read_gexf(code.to_string()).unwrap();
    assert_eq!(Ok(code.to_string()), ogdf_rs::io::writer::write_gexf(gr));
}

#[test]
fn graph_ml() {
    // let code = "<?xml version=\"1.0\"?>\n<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns&#10;http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">\n\t<graph id=\"G\" edgedefault=\"directed\">\n\t\t<node id=\"0\" />\n\t\t<node id=\"1\" />\n\t\t<edge id=\"0\" source=\"0\" target=\"1\" />\n\t</graph>\n</graphml>\n";
    // let gr = ogdf_rs::io::reader::read_graph_ml(code.to_string()).unwrap();
    // assert_eq!(
    //     Ok(code.to_string()),
    //     ogdf_rs::io::writer::write_graph_ml(gr)
    // );

    let code = "<?xml version=\"1.0\"?>\n<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns&#10;http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">\n\t<graph id=\"G\" edgedefault=\"directed\">\n\t\t<node id=\"0\" />\n\t\t<node id=\"1\" />\n\t\t<node id=\"2\" />\n\t\t<edge id=\"0\" source=\"0\" target=\"1\" />\n\t</graph>\n</graphml>\n";
    let gr = ogdf_rs::io::reader::read_graph_ml(code.to_string()).unwrap();
    assert_eq!(
        Ok(code.to_string()),
        ogdf_rs::io::writer::write_graph_ml(gr)
    );
}

#[test]
fn leda() {
    let code = "LEDA.GRAPH\nvoid\nvoid\n-1\n2\n|{}|\n|{}|\n0\n";
    let gr = ogdf_rs::io::reader::read_leda(code.to_string()).unwrap();
    assert_eq!(Ok(code.to_string()), ogdf_rs::io::writer::write_leda(gr));
}

#[test]
fn pmdiss_graph() {
    let code = "*BEGIN unknown_name.1.0\n*GRAPH 1 0 UNDIRECTED UNWEIGHTED\n*CHECKSUM -1\n*END unknown_name.1.0\n";
    let gr = ogdf_rs::io::reader::read_pmdiss_graph(code.to_string()).unwrap();
    assert_eq!(
        Ok(code.to_string()),
        ogdf_rs::io::writer::write_pmdiss_graph(gr)
    );
}

#[test]
fn rome() {
    let code = "1 0\n#\n";
    let gr = ogdf_rs::io::reader::read_rome(code.to_string()).unwrap();
    assert_eq!(Ok(code.to_string()), ogdf_rs::io::writer::write_rome(gr));
}

#[test]
fn tlp() {
    let code = "(tlp \"2.3\"\n\t(nb_nodes 1)\n\t(nb_edges 0)\n\n\t(nodes 0)\n)\n";
    let gr = ogdf_rs::io::reader::read_tlp(code.to_string()).unwrap();
    assert_eq!(Ok(code.to_string()), ogdf_rs::io::writer::write_tlp(gr));
}
