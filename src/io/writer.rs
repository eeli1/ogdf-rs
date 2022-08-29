use crate::error::Error;
use crate::graph_raw::GraphRaw;
use crate::helper::char_ptr_2_string;
use std::os::raw::c_char;

extern "C" {
    fn writeChaco(gr: GraphRaw) -> *const c_char;
    fn writeDL(gr: GraphRaw) -> *const c_char;

    fn writeDOT(gr: GraphRaw) -> *const c_char;
    fn writeGDF(gr: GraphRaw) -> *const c_char;
    fn writeGEXF(gr: GraphRaw) -> *const c_char;
    fn writeGML(gr: GraphRaw) -> *const c_char;
    fn writeGraphML(gr: GraphRaw) -> *const c_char;
    fn writeLEDA(gr: GraphRaw) -> *const c_char;
    fn writePMDissGraph(gr: GraphRaw) -> *const c_char;
    fn writeRome(gr: GraphRaw) -> *const c_char;
    fn writeTLP(gr: GraphRaw) -> *const c_char;
}

///This simple graph format is used by graph partitioning tools like Chaco, Metis, or Jostle. Its specification is described in the <a href="http://staffweb.cms.gre.ac.uk/~wc06/jostle/jostle-exe.pdf"> Jostle User Guide</a>.
pub fn write_chaco(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeChaco(gr)) }
}
///
pub fn write_dl(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeDL(gr)) }
}
///
pub fn write_dot(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeDOT(gr)) }
}
///
pub fn write_gdf(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeGDF(gr)) }
}
///
pub fn write_gexf(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeGEXF(gr)) }
}
///The GML (<i>%Graph Modelling Language</i>) file format is an Ascii-based format that has been developed by Michael Himsolt at the University of Passau. Its full specification can be found in <a href="http://www.fim.uni-passau.de/fileadmin/files/lehrstuhl/brandenburg/projekte/gml/gml-technical-report.pdf">this technical report</a>.
pub fn write_gml(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeGML(gr)) }
}
///
pub fn write_graph_ml(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeGraphML(gr)) }
}
///The LEDA graph format is a simple, Ascii-based file format used by the <a href="http://www.algorithmic-solutions.com/leda/">LEDA library</a>. Its specification is described in the <a href="http://www.algorithmic-solutions.info/leda_guide/graphs/leda_native_graph_fileformat.html"> LEDA Guide</a>.
pub fn write_leda(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeLEDA(gr)) }
}
///This simple graph format has a leading line stating the name of the graph and a following line stating the size of the graph: <pre> BEGIN unknown_name.numN.numE GRAPH numN numE UNDIRECTED UNWEIGHTED </pre>
pub fn write_pmdiss_graph(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writePMDissGraph(gr)) }
}
///The Rome-Lib format contains n "node-lines", 1 "separator-line", m "edge-lines" (in this order). These lines are as follows (whereby all IDs are integer numbers): - <b>node-line:</b> <i>NodeId</i> <tt>0</TT> - <b>separator-line:</b> starts with a <tt>#</tt>-sign - <b>edge-line:</b> <i>EdgeId</i> <tt>0</tt> <i>SourceNodeId</i> <i>TargetNodeId</i>
pub fn write_rome(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeRome(gr)) }
}
///
pub fn write_tlp(gr: GraphRaw) -> Result<String, Error> {
    unsafe { char_ptr_2_string(writeTLP(gr)) }
}
