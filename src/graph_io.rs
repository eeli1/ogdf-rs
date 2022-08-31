use crate::error::Error;
use crate::graph_raw::GraphRaw;
use crate::helper::char_ptr_2_string;
use std::ffi::CString;
use std::os::raw::c_char;

pub struct GraphIO {}

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    /// This simple graph format is used by graph partitioning tools like Chaco, Metis, or Jostle. Its specification is described in the <a href="http://staffweb.cms.gre.ac.uk/~wc06/jostle/jostle-exe.pdf"> Jostle User Guide</a>.
    Chaco,
    DL,
    /// A maximum flow instance in DIMACS format.
    DMF,
    DOT,
    GDF,
    GEXF,
    /// The GML (<i>%Graph Modelling Language</i>) file format is an Ascii-based format that has been developed by Michael Himsolt at the University of Passau. Its full specification can be found in <a href="http://www.fim.uni-passau.de/fileadmin/files/lehrstuhl/brandenburg/projekte/gml/gml-technical-report.pdf">this technical report</a>. The GML format stores the basic graph structure, i.e., nodes and edges.
    GML,
    GraphML,
    /// The LEDA graph format is a simple, Ascii-based file format used by the <a href="http://www.algorithmic-solutions.com/leda/">LEDA library</a>. Its specification is described in the <a href="http://www.algorithmic-solutions.info/leda_guide/graphs/leda_native_graph_fileformat.html"> LEDA Guide</a>.
    LEDA,
    MatrixMarket,
    /// This simple graph format has a leading line stating the name of the graph and a following line stating the size of the graph: <pre> BEGIN unknown_name.numN.numE GRAPH numN numE UNDIRECTED UNWEIGHTED </pre>
    PMDissGraph,
    /// The Rome-Lib format contains n "node-lines", 1 "separator-line", m "edge-lines" (in this order). These lines are as follows (whereby all IDs are integer numbers): - <b>node-line:</b> <i>NodeId</i> <tt>0</TT> - <b>separator-line:</b> starts with a <tt>#</tt>-sign - <b>edge-line:</b> <i>EdgeId</i> <tt>0</tt> <i>SourceNodeId</i> <i>TargetNodeId</i>
    Rome,
    Rudy,
    STP,
    TLP,
    TsplibXml,
}

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

impl GraphIO {
    pub fn read_str(str: String, file_type: FileType) -> Result<GraphRaw, Error> {
        if let Ok(c_str) = CString::new(str) {
            let char_ptr: *const c_char = c_str.as_ptr() as *const c_char;
            unsafe {
                Ok(match file_type {
                    FileType::Chaco => readChaco(char_ptr),
                    FileType::DL => readDL(char_ptr),
                    FileType::DMF => readDMF(char_ptr),
                    FileType::DOT => readDOT(char_ptr),
                    FileType::GDF => readGDF(char_ptr),
                    FileType::GEXF => readGEXF(char_ptr),
                    FileType::GML => readGML(char_ptr),
                    FileType::GraphML => readGraphML(char_ptr),
                    FileType::LEDA => readLEDA(char_ptr),
                    FileType::MatrixMarket => readMatrixMarket(char_ptr),
                    FileType::PMDissGraph => readPMDissGraph(char_ptr),
                    FileType::Rome => readRome(char_ptr),
                    FileType::Rudy => readRudy(char_ptr),
                    FileType::STP => readSTP(char_ptr),
                    FileType::TLP => readTLP(char_ptr),
                    FileType::TsplibXml => readTsplibXml(char_ptr),
                })
            }
        } else {
            Err(Error::NulError)
        }
    }

    pub fn write_str(gr: GraphRaw, file_type: FileType) -> Result<String, Error> {
        unsafe {
            char_ptr_2_string(match file_type {
                FileType::Chaco => writeChaco(gr),
                FileType::DL => writeDL(gr),
                FileType::DOT => writeDOT(gr),
                FileType::GDF => writeGDF(gr),
                FileType::GEXF => writeGEXF(gr),
                FileType::GML => writeGML(gr),
                FileType::GraphML => writeGraphML(gr),
                FileType::LEDA => writeLEDA(gr),
                FileType::PMDissGraph => writePMDissGraph(gr),
                FileType::Rome => writeRome(gr),
                FileType::TLP => writeTLP(gr),
                _ => {
                    return Err(Error::Msg(format!(
                        "FileType {:?} does not support write_str",
                        file_type
                    )))
                }
            })
        }
    }
}
