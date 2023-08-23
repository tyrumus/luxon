use crate::utils::LuxonCommand;
// use crate::utils::xdg;
use clap::Parser;
use miette::Result;
// use git2::Repository;
// use regex::Regex;

/// Init Luxon with cloned or empty repository
#[derive(Debug, Parser)]
pub struct Init {
    /// can be blank for new, empty repository;
    /// or username on github, gitlab, or sourcehut;
    /// or partial URL for github, gitlab, or sourcehut;
    /// or full URL for any other service that supports git
    url: Option<String>,
}

impl LuxonCommand for Init {
    fn exec(self) -> Result<()> {
        // TODO: implement application logic
        // let repo_path = xdg::get_repo_dir();
        println!("Do init stuff");
            // if let Ok(_) = Repository::open(&repo_path) {
            //     panic!("Repository already exists!");
            // } else {
            //     match self.url {
            //         Some(url) => {
            //             // attempt to parse URL
            //             // URL parsing regex + tests
            //             // username:
            //             // ^([-0-9A-Za-z\~]+)$
            //             //
            //             // tyrumus
            //             let re_url_username = Regex::new(r"^([-0-9A-Za-z]+)$").unwrap();
            //             //
            //             // username/reponame:
            //             // ^([-0-9A-Za-z\~]+)\/([-0-9A-Za-z\~\.]+)(\.git)?$
            //             // tyrumus/dots
            //             // tyrumus/dots.git
            //             let re_url_reponame = Regex::new(r"^([-0-9A-Za-z~]+)/([-0-9A-Za-z~.]+)(.git)?$").unwrap();
            //             //
            //             // http username:
            //             // ^(https?:\/\/)([-0-9A-Za-z\.]+)(\/[-0-9A-Za-z\~]+)$
            //             //
            //             // https://github.com/tyrumus
            //             // http://github.com/tyrumus
            //             // https://gitlab.com/tyrumus
            //             // http://gitlab.com/tyrumus
            //             // https://github.com/tyrumus
            //             // http://github.com/tyrumus
            //             // https://git.sr.ht/~tyrumus
            //             // http://git.sr.ht/~tyrumus
            //             let re_url_http_username = Regex::new("^(https?://)([-0-9A-Za-z.]+)(/[-0-9A-Za-z~]+)$").unwrap();
            //             //
            //             // http remote repo:
            //             // ^(https?:\/\/)([-0-9A-Za-z\.]+)(\/[-0-9A-Za-z\~]+)\/([-0-9A-Za-z\~]+)(\.git)?$
            //             //
            //             // https://github.com/tyrumus/repothing
            //             // https://github.com/tyrumus/repothing.git
            //             // http://github.com/tyrumus/repothing
            //             // http://github.com/tyrumus/repothing.git
            //             // https://gitlab.com/tyrumus/repothing
            //             // https://gitlab.com/tyrumus/repothing.git
            //             // http://gitlab.com/tyrumus/repothing
            //             // http://gitlab.com/tyrumus/repothing.git
            //             // https://github.com/tyrumus/repothing
            //             // https://github.com/tyrumus/repothing.git
            //             // http://github.com/tyrumus/repothing
            //             // http://github.com/tyrumus/repothing.git
            //             // https://git.sr.ht/~tyrumus/repothing
            //             // https://git.sr.ht/~tyrumus/repothing.git
            //             // http://git.sr.ht/~tyrumus/repothing
            //             // http://git.sr.ht/~tyrumus/repothing.git
            //             let re_url_http = Regex::new(r"^(https?://)?([-0-9A-Za-z.]+)(/[-0-9A-Za-z~]+)?/([-0-9A-Za-z~]+)(.git)?$").unwrap();
            //             //
            //             // ssh username:
            //             // ^([-0-9A-Za-z\.]+)@([-0-9A-Za-z\.]+):([-0-9A-Za-z\.\~]+)$
            //             // git@github.com:tyrumus
            //             // git@gitlab.com:tyrumus
            //             // git@git.sr.ht:~tyrumus
            //             let re_url_ssh_username = Regex::new("^([-0-9A-Za-z.]+)@([-0-9A-Za-z.]+):([-0-9A-Za-z.~]+)$").unwrap();
            //             //
            //             // ssh remote repo:
            //             // ^([-0-9A-Za-z\.]+)@([-0-9A-Za-z\.]+):([-0-9A-Za-z\.\~]+)(\/[-0-9A-Za-z\.\~]+)?$
            //             //
            //             // git@github.com:tyrumus/dots
            //             // git@github.com:tyrumus/dots.git
            //             // git@gitlab.com:tyrumus/dots
            //             // git@gitlab.com:tyrumus/dots.git
            //             // git@git.sr.ht:~tyrumus/dots
            //             // git@git.sr.ht:~tyrumus/dots.git
            //             let re_url_ssh = Regex::new(r"^([-0-9A-Za-z.]+)@([-0-9A-Za-z.]+):([-0-9A-Za-z.~]+)(/[-0-9A-Za-z.~]+)$").unwrap();
            //
            //             let mut http_urls: [&str; 3] = Default::default();
            //             http_urls[0] = "https://github.com/";
            //             http_urls[1] = "https://gitlab.com/";
            //             http_urls[2] = "https://git.sr.ht/~";
            //
            //             let mut ssh_urls: [&str; 3] = Default::default();
            //             ssh_urls[0] = "git@github.com:";
            //             ssh_urls[1] = "git@gitlab.com:";
            //             ssh_urls[2] = "git@git.sr.ht:~";
            //
            //             // build the huge ass proxy shit. This unfortunately does not support SOCKS
            //             // proxies for SSH. Just HTTPS proxies. That is, until libgit2 supports
            //             // SOCKS.
            //             let mut p = git2::ProxyOptions::new();
            //             p.auto();
            //
            //             let mut fo = git2::FetchOptions::new();
            //             fo.proxy_options(p);
            //
            //             let mut builder = git2::build::RepoBuilder::new();
            //             builder.fetch_options(fo);
            //
            //             println!("Initializing repo from {}", url);
            //             let url_str = url.as_str();
            //             if re_url_username.is_match(url_str) {
            //                 println!("Do HTTP URL guess from username");
            //
            //                 for prefix in http_urls.iter() {
            //                     let assem_url = format!("{}{}/dotfiles.git", prefix, url_str);
            //                     println!("trying {}", assem_url);
            //
            //                     let _repo = match builder.clone(assem_url.as_str(), &repo_path) {
            //                         Ok(repo) => repo,
            //                         Err(e) => panic!("error cloning: {}", e),
            //                     };
            //                 }
            //             } else if re_url_reponame.is_match(url_str) {
            //                 println!("Do HTTP URL guess from reponame");
            //
            //                 for prefix in http_urls.iter() {
            //                     let assem_url = format!("{}{}", prefix, url_str);
            //                     println!("trying {}", assem_url);
            //
            //                     let _repo = match builder.clone(assem_url.as_str(), &repo_path) {
            //                         Ok(repo) => repo,
            //                         Err(e) => panic!("error cloning: {}", e),
            //                     };
            //                 }
            //             } else if re_url_http_username.is_match(url_str) {
            //                 println!("Do HTTP URL guess with dotfiles.git");
            //
            //                 let assem_url = format!("{}/dotfiles.git", url_str);
            //                 println!("trying {}", assem_url);
            //
            //                 let _repo = match builder.clone(assem_url.as_str(), &repo_path) {
            //                     Ok(repo) => repo,
            //                     Err(e) => panic!("error cloning: {}", e),
            //                 };
            //             } else if re_url_http.is_match(url_str) {
            //                 println!("Do HTTP URL guess from stuff");
            //
            //                 let assem_url = format!("{}", url_str);
            //                 println!("trying {}", assem_url);
            //
            //                 let _repo = match builder.clone(assem_url.as_str(), &repo_path) {
            //                     Ok(repo) => repo,
            //                     Err(e) => panic!("error cloning: {}", e),
            //                 };
            //             } else if re_url_ssh_username.is_match(url_str) {
            //                 println!("Do SSH remote guess from username");
            //
            //                 let assem_url = format!("{}/dotfiles.git", url_str);
            //                 println!("trying {}", assem_url);
            //
            //                 let _repo = match builder.clone(assem_url.as_str(), &repo_path) {
            //                     Ok(repo) => repo,
            //                     Err(e) => panic!("error cloning: {}", e),
            //                 };
            //             } else if re_url_ssh.is_match(url_str) {
            //                 println!("Do SSH remote guess from stuff");
            //                 let assem_url = format!("{}", url_str);
            //                 println!("trying {}", assem_url);
            //
            //                 let _repo = match builder.clone(assem_url.as_str(), &repo_path) {
            //                     Ok(repo) => repo,
            //                     Err(e) => panic!("error cloning: {}", e),
            //                 };
            //             } else {
            //                 println!("Failed to parse repository URI");
            //             }
            //         },
            //         None => {
            //             println!("Initializing empty repo");
            //             let _repo = match Repository::init(repo_path) {
            //                 Ok(repo) => repo,
            //                 Err(e) => panic!("failed to init repo: {}", e),
            //             };
            //         },
            //     }
            // }
        Ok(())
    }
}
