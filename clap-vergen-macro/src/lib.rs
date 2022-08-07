use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro]
pub fn print(input: TokenStream) -> TokenStream {
    print_impl(input.into()).into()
}

fn print_impl(input: TokenStream2) -> TokenStream2 {
    quote! {
        let info = ::clap_vergen::VergenInfo {
            build_semver: env!("VERGEN_BUILD_SEMVER").to_string(),
            build_timestamp: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
            rustc_channel: env!("VERGEN_RUSTC_CHANNEL").to_string(),
            rustc_commit_date: env!("VERGEN_RUSTC_COMMIT_DATE").to_string(),
            rustc_commit_hash: env!("VERGEN_RUSTC_COMMIT_HASH").to_string(),
            rustc_host_triple: env!("VERGEN_RUSTC_HOST_TRIPLE").to_string(),
            rustc_llvm_version: env!("VERGEN_RUSTC_LLVM_VERSION").to_string(),
            rustc_semver: env!("VERGEN_RUSTC_SEMVER").to_string(),
            cargo_features: env!("VERGEN_CARGO_FEATURES").to_string(),
            cargo_profile: env!("VERGEN_CARGO_PROFILE").to_string(),
            cargo_target_triple: env!("VERGEN_CARGO_TARGET_TRIPLE").to_string(),
            git_branch: env!("VERGEN_GIT_BRANCH").to_string(),
            git_commit_timestamp: env!("VERGEN_GIT_COMMIT_TIMESTAMP").to_string(),
            git_semver: env!("VERGEN_GIT_SEMVER").to_string(),
            git_sha: env!("VERGEN_GIT_SHA").to_string(),
        };
        if #input.json {
            println!(
                "{}",
                info.to_json().expect("Fail to generate version JSON")
            );
        } else {
            println!("{}", info);
        }
    } // quote
}
