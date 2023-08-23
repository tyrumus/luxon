use crate::utils::LuxonCommand;
// use crate::utils::xdg;
use clap::Parser;
use miette::Result;
// use git2::Repository;

// yoinked from git2 example
// use std::io::{self, Write};

/// Pull changes from remote repo, but don't apply
#[derive(Debug, Parser)]
pub struct Pull {
    #[arg(short, long)]
    init: bool,
}

impl LuxonCommand for Pull {
    fn exec(self) -> Result<()> {
        println!("Pulling changes from remote repo");
        // let repo_path = xdg::get_repo_dir();
        //     if let Ok(repo) = Repository::open(&repo_path) {
        //         // do fetch
        //         let mut remote = repo.find_remote("origin").unwrap();
        //         let remote_branch = "master";
        //
        //         // attempt merge
        //         let mut cb = git2::RemoteCallbacks::new();
        //         cb.transfer_progress(|stats| {
        //             if stats.received_objects() == stats.total_objects() {
        //                 print!("Resolving deltas {}/{}\r", stats.indexed_deltas(), stats.total_deltas());
        //             } else if stats.total_objects() > 0 {
        //                 print!("Received {}/{} objects ({}) in {} bytes\r", stats.received_objects(), stats.total_objects(), stats.indexed_objects(), stats.received_bytes());
        //             }
        //             io::stdout().flush().unwrap();
        //             true
        //         });
        //         let mut p = git2::ProxyOptions::new();
        //         p.auto();
        //
        //         let mut fo = git2::FetchOptions::new();
        //         fo.remote_callbacks(cb);
        //         fo.proxy_options(p);
        //         fo.download_tags(git2::AutotagOption::All);
        //         println!("Fetching {} for repo", remote.name().unwrap());
        //         remote.fetch(&[remote_branch], Some(&mut fo), None).unwrap();
        //
        //         let stats = remote.stats();
        //         if stats.local_objects() > 0 {
        //             println!("\rReceived {}/{} objects in {} bytes (used {} local objects)",
        //                 stats.indexed_objects(),
        //                 stats.total_objects(),
        //                 stats.received_bytes(),
        //                 stats.local_objects()
        //             );
        //         } else {
        //             println!("\rReceived {}/{} objects in {} bytes",
        //                 stats.indexed_objects(),
        //                 stats.total_objects(),
        //                 stats.received_bytes()
        //             );
        //         }
        //
        //         let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
        //         let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
        //
        //         // merge analysis
        //         let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();
        //
        //         if analysis.0.is_fast_forward() {
        //             let refname = format!("refs/heads/{}", remote_branch);
        //             match repo.find_reference(&refname){
        //                 Ok(mut r) => {
        //                     // fast_forward
        //                     let name = match r.name() {
        //                         Some(s) => s.to_string(),
        //                         None => String::from_utf8_lossy(r.name_bytes()).to_string(),
        //                     };
        //                     let msg = format!("Fast-Forward: Setting {} to id: {}", name, fetch_commit.id());
        //                     println!("{}", msg);
        //                     r.set_target(fetch_commit.id(), &msg).unwrap();
        //                     repo.set_head(&name).unwrap();
        //                     repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force())).unwrap();
        //                 }
        //                 Err(_) => {
        //                     repo.reference(&refname, fetch_commit.id(), true, &format!("Setting {} to {}", remote_branch, fetch_commit.id())).unwrap();
        //                     repo.set_head(&refname).unwrap();
        //                     repo.checkout_head(Some(
        //                             git2::build::CheckoutBuilder::default()
        //                             .allow_conflicts(true)
        //                             .conflict_style_merge(true)
        //                             .force()
        //                     )).unwrap();
        //                 }
        //             };
        //         } else if analysis.0.is_normal() {
        //             let head_commit = repo.reference_to_annotated_commit(&repo.head().unwrap()).unwrap();
        //
        //             // do normal_merge
        //             let local_tree = repo.find_commit(head_commit.id()).unwrap().tree().unwrap();
        //             let remote_tree = repo.find_commit(fetch_commit.id()).unwrap().tree().unwrap();
        //             let ancestor = repo.find_commit(repo.merge_base(head_commit.id(), fetch_commit.id()).unwrap()).unwrap().tree().unwrap();
        //             let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None).unwrap();
        //
        //             if idx.has_conflicts() {
        //                 println!("Merge conflicts detected...");
        //                 repo.checkout_index(Some(&mut idx), None).unwrap();
        //             } else {
        //                 let result_tree = repo.find_tree(idx.write_tree_to(&repo).unwrap()).unwrap();
        //                 let msg = format!("Merge: {} into {}", fetch_commit.id(), head_commit.id());
        //                 let sig = repo.signature().unwrap();
        //                 let local_commit = repo.find_commit(head_commit.id()).unwrap();
        //                 let remote_commit = repo.find_commit(fetch_commit.id()).unwrap();
        //                 let _merge_commit = repo.commit(Some("HEAD"), &sig, &sig, &msg, &result_tree, &[&local_commit, &remote_commit]).unwrap();
        //                 repo.checkout_head(None).unwrap();
        //             }
        //         } else {
        //             println!("nothing to do");
        //         }
        //     }
        if self.init {
            // TODO: implement application logic
            println!("Regenerating repository config");
            unimplemented!();
        }
        Ok(())
    }
}
