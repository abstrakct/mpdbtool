#[cfg(test)]
mod tests {
    use crate::Setlists;
    use serde_xml_rust::from_str;

    // const SAMPLE_XML: &str = r#"
    // <setlists>
    //     <setlist status="confirmed" eventDate="16-05-2021">
    //         <sets>
    //             <set>
    //                 <song name="The Transmutation of Cosmoctopus Lurker"/>
    //                 <song name="N.O.X."/>
    //             </set>
    //         </sets>
    //     </setlist>
    // </setlists>
    // "#;

    // #[test]
    // #[ignore]
    // fn test_valid_xml_deserialization() {
    //     let setlists: Setlists = from_str(SAMPLE_XML).expect("Failed to deserialize XML");

    //     assert_eq!(setlists.data.len(), 1);
    //     let setlist = &setlists.data[0];

    //     let sets = setlist.sets.set;

    //     assert_eq!(sets.len(), 1);
    //     let set = &sets[0];

    //     assert_eq!(set.songs.len(), 2);
    //     assert_eq!(set.songs[0].name, "The Transmutation of Cosmoctopus Lurker");
    //     assert_eq!(set.songs[1].name, "N.O.X.");
    // }

    #[test]
    fn test_invalid_xml_format() {
        let xml = r#"<setlists><setlist><invalid></setlists>"#;

        let result: Result<Setlists, _> = from_str(xml);
        assert!(result.is_err()); // Ensure invalid XML fails
    }
}
