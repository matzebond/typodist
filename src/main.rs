extern crate clap;
extern crate structopt;

extern crate typodist_lib;
use typodist_lib as lib;
use structopt::StructOpt;
use structopt::clap::AppSettings;
use clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    enum Layout {
        QWERTZ,
        QWERTZ_ANYSOFT_EXTRA,
    }
}

#[derive(StructOpt, Debug)]
struct OptMetric {
    ///Layout to use for distance metric.
    #[structopt(short = "l", long = "layout", default_value = "QWERTZ", raw(possible_values = "&Layout::variants()", case_insensitive = "true"))]
    layout: Layout,

    ///Use a simpler distance metric which treats insertion, deletion, substitution and transposition equally
    #[structopt(long = "simple-metric")]
    simple: bool,

    //
    #[structopt(long = "mobile-metric")]
    mobile: bool,
}

#[derive(StructOpt, Debug)]
struct OptDistance {
    #[structopt()]
    target: String,

    #[structopt()]
    other: String,

    #[structopt(flatten)]
    metric: OptMetric,
}

#[derive(StructOpt, Debug)]
struct OptGenerate {
    ///input to generate "near" typos
    #[structopt()]
    input: String,

    ///maximum distance of the generated words
    #[structopt(short = "m", long = "max-dist", default_value = "1.1")]
    max_dist: f32,

    ///print detailed information about each generated typo (with metric distance)
    #[structopt(long)]
    detailed: bool,

    #[structopt(flatten)]
    metric: OptMetric,
}

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "AppSettings::InferSubcommands"))]
enum Command {
    #[structopt(name = "dist")]
    Distance(OptDistance),

    #[structopt(name = "generate")]
    Generate(OptGenerate),
}


fn main() {
    let opt = Command::from_args();

    use Command::*;
    match opt {
        Distance(opt) => distance(opt),
        Generate(opt) => generate(opt),
    }
}

fn distance(opt: OptDistance) {
    let metric = get_metric(opt.metric);
    println!("{}", lib::distance(&opt.target, &opt.other, &metric));
}

fn generate(opt: OptGenerate) {
    let results;
    let metric = get_metric(opt.metric);
    results = lib::generate(&opt.input, opt.max_dist, &metric);

    for dcost in results {
        if opt.detailed {
            println!("{} {}", dcost.cost, dcost.word);
        } else {
            println!("{}", dcost.word);
        }
    }
}

fn get_metric(opt: OptMetric) -> lib::Metric {
    if opt.simple {
        return lib::get_simple_metric()
    }

    let layout: &lib::KeyLayout;

    match opt.layout {
        Layout::QWERTZ_ANYSOFT_EXTRA => layout = &lib::Layouts::QWERTZ_ANYSOFT_EXTRA,
        Layout::QWERTZ => layout = &lib::Layouts::QWERTZ,
    }

    if opt.mobile {
        lib::get_layout_metric_mobile(&layout)
    } else {
        lib::get_layout_metric(&layout)
    }
}
