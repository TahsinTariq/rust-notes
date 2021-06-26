extern crate tch;
use tch::Tensor;

fn main() {
    let t = Tensor::of_slice(&[3, 1, 4, 1, 5]);
    let t = t * 2;
    t.print();
    println!("{}", tch::Cuda::is_available());

    // if use_cuda {
    //     println!("cargo:rustc-link-lib=torch_cuda");
    //     println!("cargo:rustc-link-lib=c10_cuda");
    // }
}
