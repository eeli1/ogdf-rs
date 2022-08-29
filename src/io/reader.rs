use crate::error::Error;
use crate::graph_raw::GraphRaw;
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn readChaco(str: *const c_char) -> GraphRaw;
    fn readDL(str: *const c_char) -> GraphRaw;
    fn readDMF(str: *const c_char) -> GraphRaw;
    fn readDOT(str: *const c_char) -> GraphRaw;
    fn readGDF(str: *const c_char) -> GraphRaw;
    fn readGEXF(str: *const c_char) -> GraphRaw;
    fn readGML(str: *const c_char) -> GraphRaw;
    fn readGraphML(str: *const c_char) -> GraphRaw;
    fn readLEDA(str: *const c_char) -> GraphRaw;
    fn readMatrixMarket(str: *const c_char) -> GraphRaw;
    fn readPMDissGraph(str: *const c_char) -> GraphRaw;
    fn readRome(str: *const c_char) -> GraphRaw;
    fn readRudy(str: *const c_char) -> GraphRaw;
    fn readSTP(str: *const c_char) -> GraphRaw;
    fn readTLP(str: *const c_char) -> GraphRaw;
    fn readTsplibXml(str: *const c_char) -> GraphRaw;
}
#[inline]
fn read(
    str: String,
    func: unsafe extern "C" fn(str: *const c_char) -> GraphRaw,
) -> Result<GraphRaw, Error> {
    if let Ok(c_str) = CString::new(str) {
        let char_ptr: *const c_char = c_str.as_ptr() as *const c_char;
        unsafe { Ok(func(char_ptr)) }
    } else {
        Err(Error::NulError)
    }
}
///This simple graph format is used by graph partitioning tools like Chaco, Metis, or Jostle. Its specification is described in the <a href="http://staffweb.cms.gre.ac.uk/~wc06/jostle/jostle-exe.pdf"> Jostle User Guide</a>.
pub fn read_chaco(str: String) -> Result<GraphRaw, Error> {
    read(str, readChaco)
}
///
pub fn read_dl(str: String) -> Result<GraphRaw, Error> {
    read(str, readDL)
}
///Reads a maximum flow instance in DIMACS format.
pub fn read_dmf(str: String) -> Result<GraphRaw, Error> {
    read(str, readDMF)
}
///
pub fn read_dot(str: String) -> Result<GraphRaw, Error> {
    read(str, readDOT)
}
///
pub fn read_gdf(str: String) -> Result<GraphRaw, Error> {
    read(str, readGDF)
}
///
pub fn read_gexf(str: String) -> Result<GraphRaw, Error> {
    read(str, readGEXF)
}
///The GML (<i>%Graph Modelling Language</i>) file format is an Ascii-based format that has been developed by Michael Himsolt at the University of Passau. Its full specification can be found in <a href="http://www.fim.uni-passau.de/fileadmin/files/lehrstuhl/brandenburg/projekte/gml/gml-technical-report.pdf">this technical report</a>.
pub fn read_gml(str: String) -> Result<GraphRaw, Error> {
    read(str, readGML)
}
///
pub fn read_graph_ml(str: String) -> Result<GraphRaw, Error> {
    read(str, readGraphML)
}
///The LEDA graph format is a simple, Ascii-based file format used by the <a href="http://www.algorithmic-solutions.com/leda/">LEDA library</a>. Its specification is described in the <a href="http://www.algorithmic-solutions.info/leda_guide/graphs/leda_native_graph_fileformat.html"> LEDA Guide</a>.
pub fn read_leda(str: String) -> Result<GraphRaw, Error> {
    read(str, readLEDA)
}
///
pub fn read_matrix_market(str: String) -> Result<GraphRaw, Error> {
    read(str, readMatrixMarket)
}
///This simple graph format has a leading line stating the name of the graph and a following line stating the size of the graph: <pre> BEGIN unknown_name.numN.numE GRAPH numN numE UNDIRECTED UNWEIGHTED </pre>
pub fn read_pmdiss_graph(str: String) -> Result<GraphRaw, Error> {
    read(str, readPMDissGraph)
}
///The Rome-Lib format contains n "node-lines", 1 "separator-line", m "edge-lines" (in this order). These lines are as follows (whereby all IDs are integer numbers): - <b>node-line:</b> <i>NodeId</i> <tt>0</TT> - <b>separator-line:</b> starts with a <tt>#</tt>-sign - <b>edge-line:</b> <i>EdgeId</i> <tt>0</tt> <i>SourceNodeId</i> <i>TargetNodeId</i>
pub fn read_rome(str: String) -> Result<GraphRaw, Error> {
    read(str, readRome)
}
///
pub fn read_rudy(str: String) -> Result<GraphRaw, Error> {
    read(str, readRudy)
}
///Reads a graph in SteinLib format
pub fn read_stp(str: String) -> Result<GraphRaw, Error> {
    read(str, readSTP)
}
///
pub fn read_tlp(str: String) -> Result<GraphRaw, Error> {
    read(str, readTLP)
}
///
pub fn read_tsplib_xml(str: String) -> Result<GraphRaw, Error> {
    read(str, readTsplibXml)
}
