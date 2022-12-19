use clap::Parser;
use MyKmeans;

/// Split a pair of BD rhapsody fastq files (R1 and R2) into sample specific fastq pairs
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Stefan L. <stefan.lang@med.lu.se>")]
struct Opts {
    /// the data you want to cluster
    #[clap(short, long)]
    file: String,
    /// how many clusters to create
    #[clap(short, long)]
    k: usize,
    /// the file to store the clusters in
    #[clap(short, long)]
    outfile: String,
}


fn main() {

    let opts: Opts = Opts::parse();

    let data = MyKmeans::read_tsv( &opts.file, 17, 256 );

}
