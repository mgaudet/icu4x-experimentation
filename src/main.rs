use icu_provider_fs::FsDataProvider;
use icu_uniset::{enum_props, props, UnicodeSetBuilder};

fn main() {
    let mut builder = UnicodeSetBuilder::new();
    builder.add_range(&('A'..'C'));
    let set = builder.build();
    println!("Set contains A: {}", set.contains('A'));

    let data_provider = FsDataProvider::try_new("/home/matthew/icu4x-data").expect("Provider");

    // So, the old PPCUD parser apparently doesn't yet parse the general category values. So we can't test that yet.
    // let letter_set =
    //     props::get_general_category_val_set(&data_provider, enum_props::GeneralCategory::Letter)
    //         .expect("Didn't get set");

    // println!("Set contains A: {}", letter_set.contains('A'));
    // println!("Set contains -: {}", letter_set.contains('-'));

    let wspace_set = props::get_white_space_property(&data_provider).expect("Didn't get set");
    println!("Set contains ' ': {}", wspace_set.contains(' '));
    println!("Set contains '\\n': {}", wspace_set.contains('\n'));
    println!("Set contains 'X': {}", wspace_set.contains('X'));
}
