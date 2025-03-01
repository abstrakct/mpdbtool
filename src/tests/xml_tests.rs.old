#[cfg(test)]
mod tests {
    use crate::Setlists;
    use serde_xml_rust::from_str;

    const SAMPLE_XML: &str = r#"
    <setlists>
        <setlist status="confirmed" eventDate="16-05-2021">
        <artist name="Motorpsycho"/>
        <venue name="Verkstedhallen">
            <city name="Trondheim">
                <country name="Norway"/>
            </city>
        </venue>
        <sets>
            <set>
                <song name="The Transmutation of Cosmoctopus Lurker"/>
                <song name="N.O.X."/>
            </set>
        </sets>
        </setlist>
    </setlists>
    "#;

    #[test]
    fn test_valid_xml_deserialization() {
        let testdata: Setlists = from_str(SAMPLE_XML).expect("Failed to deserialize XML");

        assert_eq!(testdata.data.len(), 1);

        let setlist = &testdata.data[0];
        assert_eq!(setlist.sets.set.len(), 1);

        let set = &setlist.sets.set[0];

        assert_eq!(set.songs.len(), 2);
        assert_eq!(set.songs[0].name, "The Transmutation of Cosmoctopus Lurker");
        assert_eq!(set.songs[1].name, "N.O.X.");

        assert_eq!(setlist.artist.name, "Motorpsycho");
        assert_eq!(setlist.venue.name, "Verkstedhallen");
        assert_eq!(setlist.venue.city.name, "Trondheim");
        assert_eq!(setlist.venue.city.country.name, "Norway");
    }

    #[test]
    fn test_invalid_xml_format() {
        let xml = r#"<setlists><setlist><invalid></setlists>"#;

        let result: Result<Setlists, _> = from_str(xml);
        assert!(result.is_err()); // Ensure invalid XML fails
    }
}
