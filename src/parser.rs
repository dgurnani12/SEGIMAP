pub use self::grammar::{fetch, sequence_set};

peg_file! grammar("grammar.rustpeg")

#[deriving(Clone, PartialEq, Show)]
enum SequenceItem {
    Number(uint),
    Range(Box<SequenceItem>, Box<SequenceItem>),
    All
}

#[deriving(PartialEq, Show)]
enum CommandType {
    Fetch
}

// TODO: Sort these in alphabetical order.
#[deriving(PartialEq, Show)]
enum Attribute {
    Envelope,
    Flags,
    InternalDate,
    RFC822,
    RFC822Header,
    RFC822Size,
    RFC822Text,
    Body,
    BodyStructure,
    UID,
    /*
    BODY section ("<" number "." nz_number ">")?,
    BODYPEEK section ("<" number "." nz_number ">")?
    */
}

#[deriving(PartialEq, Show)]
struct Command {
    command_type: CommandType,
    sequence_set: Vec<SequenceItem>,
    attributes: Vec<Attribute>
}

impl Command {
    pub fn new(
            command_type: CommandType,
            sequence_set: Vec<SequenceItem>,
            attributes: Vec<Attribute>) -> Command {
        Command {
            command_type: command_type,
            sequence_set: sequence_set,
            attributes: attributes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{fetch, sequence_set};
    use super::{
        All,
        Body,
        BodyStructure,
        Command,
        Envelope,
        Fetch,
        Flags,
        InternalDate,
        Number,
        Range,
        RFC822,
        RFC822Header,
        RFC822Size,
        RFC822Text,
        UID
    };

    #[test]
    fn test_invalid_sequences() {
        assert!(sequence_set("").is_err());
        assert!(sequence_set("a").is_err());
        assert!(sequence_set("0").is_err());
        assert!(sequence_set("a:*").is_err());
        assert!(sequence_set(":*").is_err());
        assert!(sequence_set("1:").is_err());
        assert!(sequence_set("1:0").is_err());
        assert!(sequence_set("0:1").is_err());
        assert!(sequence_set("4,5,6,").is_err());
    }

    #[test]
    fn test_sequence_num() {
        let seq = sequence_set("4324");
        assert!(seq.is_ok());
        let seq = seq.unwrap();
        let expected = vec![Number(4324)];
        assert_eq!(seq, expected);
    }

    #[test]
    fn test_sequence_all() {
        let seq = sequence_set("*");
        assert!(seq.is_ok());
        let seq = seq.unwrap();
        let expected = vec![All];
        assert_eq!(seq, expected);
    }

    #[test]
    fn test_sequence_ranges() {
        let seq = sequence_set("98:100");
        assert!(seq.is_ok());
        let seq = seq.unwrap();
        let expected = vec![Range(box Number(98), box Number(100))];
        assert_eq!(seq, expected);

        assert!(sequence_set("1:5").is_ok());
        assert!(sequence_set("21:44").is_ok());
    }

    #[test]
    fn test_sequence_range_all() {
        let seq = sequence_set("31:*");
        assert!(seq.is_ok());
        let seq = seq.unwrap();
        let expected = vec![Range(box Number(31), box All)];
        assert_eq!(seq, expected);
    }

    #[test]
    fn test_sequence_set() {
        let seq = sequence_set("1231,1342,12,98:104,16");
        assert!(seq.is_ok());
        let seq = seq.unwrap();
        let expected = vec![Number(1231), Number(1342), Number(12), Range(box Number(98), box Number(104)), Number(16)];
        assert_eq!(seq, expected);
    }

    #[test]
    fn test_fetch_all() {
        let cmd = fetch("FETCH 1:5 ALL");
        println!("CMD: {}", cmd);
        assert!(cmd.is_ok());
        let cmd = cmd.unwrap();
        let expected = Command::new(
                Fetch,
                vec![Range(box Number(1), box Number(5))],
                vec![Flags, InternalDate, RFC822Size, Envelope]);
        assert_eq!(cmd, expected);
    }

    #[test]
    fn test_fetch_fast() {
        let cmd = fetch("FETCH 3,5 FAST");
        println!("CMD: {}", cmd);
        assert!(cmd.is_ok());
        let cmd = cmd.unwrap();
        let expected = Command::new(
                Fetch,
                vec![Number(3), Number(5)],
                vec![Flags, InternalDate, RFC822Size]);
        assert_eq!(cmd, expected);
    }

    #[test]
    fn test_fetch_full() {
        let cmd = fetch("FETCH 2:7 FULL");
        println!("CMD: {}", cmd);
        assert!(cmd.is_ok());
        let cmd = cmd.unwrap();
        let expected = Command::new(
                Fetch,
                vec![Range(box Number(2), box Number(7))],
                vec![Flags, InternalDate, RFC822Size, Envelope, Body]);
        assert_eq!(cmd, expected);
    }

    #[test]
    fn test_complex_fetch() {
        let cmd = fetch("FETCH * (FLAGS BODY[HEADER.FIELDS (DATE FROM)])");
        println!("CMD: {}", cmd);
        assert!(cmd.is_ok());
        let cmd = cmd.unwrap();
        let expected = Command::new(
                Fetch,
                vec![All],
                Vec::new()); // TODO: Fill in the flags.
        assert_eq!(cmd, expected);
    }
}
