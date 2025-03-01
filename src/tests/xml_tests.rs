#[cfg(test)]
mod tests {
    use crate::Setlists;

    #[test]
    fn test_full_xml_can_be_parsed() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
                <setlists>
                    <setlist status="complete" eventDate="2023-01-01">
                        <artist>
                            <name>Test Artist</name>
                        </artist>
                        <venue>
                            <name>Test Venue</name>
                            <city>
                                <name>Test City</name>
                                <country>
                                    <name>Test Country</name>
                                </country>
                            </city>
                        </venue>
                        <sets>
                            <set>
                                <name>Test Set</name>
                                <song>
                                    <name>Test Song</name>
                                </song>
                            </set>
                        </sets>
                    </setlist>
                </setlists>"#;

        let result = Setlists::from_xml(xml);
        assert!(result.is_ok());

        let setlists = result.unwrap();
        assert_eq!(setlists.data.len(), 1);
        assert_eq!(setlists.data[0].status, "complete");
        assert_eq!(setlists.data[0].event_date, "2023-01-01");
        assert_eq!(setlists.data[0].artist.name, "Test Artist");
        assert_eq!(setlists.data[0].venue.name, "Test Venue");
        assert_eq!(setlists.data[0].venue.city.name, "Test City");
        assert_eq!(setlists.data[0].venue.city.country.name, "Test Country");
        assert_eq!(setlists.data[0].sets.set.len(), 1);
        assert_eq!(
            setlists.data[0].sets.set[0].name,
            Some("Test Set".to_string())
        );
        assert!(setlists.data[0].sets.set[0].songs.is_some());
        let songs = &setlists.data[0].sets.set[0].songs.as_ref().unwrap();
        assert_eq!(songs.len(), 1);
        assert_eq!(songs[0].name, "Test Song");
    }

    #[test]
    fn test_xml_without_songs_can_be_parsed() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
                <setlists>
                    <setlist status="complete" eventDate="2023-01-01">
                        <artist>
                            <name>Test Artist</name>
                        </artist>
                        <venue>
                            <name>Test Venue</name>
                            <city>
                                <name>Test City</name>
                                <country>
                                    <name>Test Country</name>
                                </country>
                            </city>
                        </venue>
                        <sets>
                            <set>
                                <name>Test Set</name>
                            </set>
                        </sets>
                    </setlist>
                </setlists>"#;

        let result = Setlists::from_xml(xml);
        assert!(result.is_ok());

        let setlists = result.unwrap();
        assert_eq!(setlists.data.len(), 1);
        assert_eq!(setlists.data[0].status, "complete");
        assert_eq!(setlists.data[0].event_date, "2023-01-01");
        assert_eq!(setlists.data[0].artist.name, "Test Artist");
        assert_eq!(setlists.data[0].venue.name, "Test Venue");
        assert_eq!(setlists.data[0].venue.city.name, "Test City");
        assert_eq!(setlists.data[0].venue.city.country.name, "Test Country");
        assert_eq!(setlists.data[0].sets.set.len(), 1);
        assert_eq!(
            setlists.data[0].sets.set[0].name,
            Some("Test Set".to_string())
        );
        assert!(setlists.data[0].sets.set[0].songs.is_none());
    }

    #[test]
    fn test_from_xml_invalid() {
        let invalid_xml = "not xml";
        let result = Setlists::from_xml(invalid_xml);
        assert!(result.is_err());
    }
}
