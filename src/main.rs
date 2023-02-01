use clap::Parser;
use k8s_openapi::api::core::v1::Pod;

use kube::{
    api::{Api, AttachParams, ListParams, ResourceExt},
    Client,
};

use tokio::io::AsyncWriteExt;

#[derive(Parser)]
#[command(name = "kubectl-distribute")]
#[command(author = "Andrew Nordman <cadwallion@gmail.com>")]
#[command(version = "0.1")]
#[command(about = "Distribute a file to multiple Kubernetes pods via label selector", long_about = None)]
struct Cli {
    /// The pod label value(s) used to select pods for distribution
    #[arg(short, long)]
    value: Vec<String>,
    /// The file to distribute to the selected pods
    #[arg(short, long)]
    file: String,
    /// The path in each of the pods to distribute the file
    #[arg(short, long)]
    path: String,
    /// The label key, used to select pods for distribution.
    #[arg(short, long, default_value_t = String::from("app"))]
    label: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let kube_client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(kube_client);

    let mut ar = tar::Builder::new(Vec::new());
    ar.append_path(&cli.file)?;
    let data = ar.into_inner()?;

    for app in cli.value {
        let lp = ListParams::default()
            .labels(&format!("{}={}", &cli.label, app))
            .timeout(10);
        for p in pods.list(&lp).await? {
            let pod_name = p.name_any();
            let ap = AttachParams::default().stdin(true).stderr(false);
            println!("Copying {} to {}:{}", &cli.file, &pod_name, &cli.path);
            let mut tar = pods
                .exec(&pod_name, vec!["tar", "xf", "-", "-C", &cli.path], &ap)
                .await?;
            tar.stdin().unwrap().write_all(&data).await?;
        }
    }

    Ok(())
}
