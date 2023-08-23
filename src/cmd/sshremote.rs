use crate::utils::LuxonCommand;
// use crate::utils::xdg;
use clap::Parser;
use miette::Result;
// use regex::Regex;
// use std::process::Command;

/// Attempt to switch git remote 'origin' from HTTPS to SSH
#[derive(Debug, Parser)]
pub struct SshRemote {
    #[arg(short, long)]
    no_check: bool,
}

impl LuxonCommand for SshRemote {
    fn exec(self) -> Result<()> {
        // let repo_path = xdg::get_repo_dir();
        println!("This is the ssh-remote command thing. It too, is not implemented.");
        // let repo = gix::discover(&repo_path).expect("no repo. suckle my butthole");
        // if let Some(Ok(remote)) = repo.find_default_remote(gix::remote::Direction::Fetch) {
        //     let (old_url, _) = remote.sanitized_url_and_version(gix::remote::Direction::Fetch).expect("error getting URL, you stupid cunt");
        //     let old_url_host = old_url.host().expect("error getting host, you are trash");
        //
        //     println!("{:?}", old_url);
        //     println!("{:?}", old_url_host);
        //     println!("{:?}", old_url.path);
        //     let new_url_string = format!("git@{}:{}", old_url_host, old_url.path);
        //     let new_url = gix::url::parse(Into::into(new_url_string.as_str())).expect("failed to generate SSH URL");
        //     println!("{:?}", new_url);
        //
        //     let data = repo.remote_at(new_url).expect("Failed to change URL");
        //
        //     println!("\n\n repo: {:?}", repo);
        //     println!("data: {:?}", data);
        //
        //     let conf = repo.config_snapshot();
        //     // TODO: WHAT THE HELL DO I DO HERE???
        //     data.save_to(conf);
        //    
        //     println!("config: {:?}", conf);
        //    
        //     // test shit
        //     let u = gix::url::parse(Into::into("git@github.com:tyrumus/dotfiles.git"));
        //     println!("test poop: {:?}", u);
        // } else {
        //     panic!("idk wtf the default remote is. gg.");
        // }
        //     // let remote_url = remote.url().unwrap();
        //     // match re_url.captures(remote_url) {
        //     //     Some(cap) => {
        //     //         println!("captures: {:?}", &cap);
        //     //         let domain = &cap[1];
        //     //         let repo_url = &cap[2];
        //     //         let new_url = format!("git@{}:{}", domain, repo_url);
        //     //
        //     //         if !self.no_check {
        //     //             println!("Checking for successful connection over SSH");
        //     //
        //     //             // not a legitimate check. Different git services respond with
        //     //             // different exit codes. Either hardcode git services, or remove check
        //     //             // entirely.
        //     //             let shell_bin = xdg::get_shell();
        //     //             let cmd_arg = format!("ssh -T git@{}", domain);
        //     //             let status = Command::new(shell_bin).args(["-c", cmd_arg.as_str()]).status();
        //     //             println!("Process finished with {:?}", status.unwrap().success());
        //     //             panic!("peepeepoopoo");
        //     //         }
        //     //
        //     //         println!("Switching from HTTPS to SSH");
        //     //         let _status = repo.remote_set_url("origin", new_url.as_str());
        //     //     },
        //     //     None => {
        //     //         println!("Failed to parse URL: {}", remote_url);
        //     //     }
        //     // };
        Ok(())
    }
}
