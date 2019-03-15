load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

#local_repository(
#    name = "io_bazel_rules_rust",
#    path = "../rules_rust",
#)
#
http_archive(
    name = "io_bazel_rules_rust",
    patches = ["rules_rust.patch"],
    sha256 = "39a45874ad3494b3eca6e30cb4ff02e12f3561456361652263c75fd405be8e63",
    strip_prefix = "rules_rust-5fa9b101a68ddd4a628462b8d2aae06c6cbbda15",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/5fa9b101a68ddd4a628462b8d2aae06c6cbbda15.zip",
    ],
)

http_archive(
    name = "bazel_skylib",
    sha256 = "eb5c57e4c12e68c0c20bc774bfbc60a568e800d025557bc4ea022c6479acc867",
    strip_prefix = "bazel-skylib-0.6.0",
    url = "https://github.com/bazelbuild/bazel-skylib/archive/0.6.0.tar.gz",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repository_set")

rust_repository_set(
    name = "rust_linux_x86_64",
    exec_triple = "x86_64-unknown-linux-gnu",
    extra_target_triples = ["thumbv7em-none-eabihf"],
    version = "1.31.0",
)

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")

bazel_version(name = "bazel_version")

http_archive(
    name = "quote",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "quote",
    srcs = glob(["src/**/*.rs"]),
	deps = ["@proc_macro2"],
    rustc_flags = ["--cfg", "integer128"],
    proc_macro_dep = True,
)
    """,
    sha256 = "123afe3021dbaf20f09d8bc3c1b1a405bab57b311a96a99d03e290ff958a4333",
    strip_prefix = "quote-0.6.11",
    url = "https://github.com/dtolnay/quote/archive/0.6.11.tar.gz",
)

http_archive(
    name = "syn",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "syn",
    srcs = glob(["src/**/*.rs"]),
	deps = ["@proc_macro2", "@unicode_xid"],
    crate_features = ["derive", "parsing", "proc-macro"],
    rustc_flags = ["--cfg", "syn_can_use_thread_id", "--cfg", "syn_can_call_macro_by_path"],
    proc_macro_dep = True,
)
    """,
    sha256 = "5ae4c18de46a0d9f99dac8c670a3b0806e23e93d21aef14b5a98174011643831",
    strip_prefix = "syn-0.15.29",
    url = "https://github.com/dtolnay/syn/archive/0.15.29.tar.gz",
)

http_archive(
    name = "proc_macro2",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "proc_macro2",
    srcs = glob(["src/**/*.rs"]),
	deps = ["@unicode_xid"],
    rustc_flags = ["--cfg", "u128", "--cfg", "use_proc_macro", "--cfg", "wrap_proc_macro"],
    proc_macro_dep = True,
)
    """,
    strip_prefix = "proc-macro2-0.4.27",
    url = "https://github.com/alexcrichton/proc-macro2/archive/0.4.27.tar.gz",
)

http_archive(
    name = "unicode_xid",
    build_file_content = """
package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "unicode_xid",
    srcs = glob(["src/**/*.rs"]),
    proc_macro_dep = True,
)
    """,
    strip_prefix = "unicode-xid-0.1.0",
    url = "https://github.com/unicode-rs/unicode-xid/archive/v0.1.0.tar.gz",
)
