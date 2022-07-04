use bagel::Gtor;

bagel::def! {
    #[derive(Gtor)]
    struct Hello {
        #[gtor_copy]
        a: u8 = 10,
    }
}

fn main() {
    assert_eq!(Hello::default().get_a(), 10);
}
