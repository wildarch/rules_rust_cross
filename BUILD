package(default_visibility = ["//visibility:public"])

load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_test")

rust_library(
    name = "hello_lib",
    srcs = ["lib.rs"],
    crate_type = "staticlib",
    edition = "2018",
    deps = [":hello_macro"],
)

rust_library(
    name = "hello_macro",
    srcs = ["hello_macro.rs"],
    crate_type = "proc-macro",
    edition = "2018",
    deps = [
        "@quote",
        "@syn",
    ],
)

rust_test(
    name = "hello_lib_test",
    deps = [":hello_lib"],
)

platform(
    name = "stm32f4",
    constraint_values = [
        "@io_bazel_rules_rust//rust/platform:none",
        "@bazel_tools//platforms:arm",
    ],
)
