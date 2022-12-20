use color_eyre::eyre::eyre;

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod test {
    use crate::find_tag_idx;

    #[test]
    fn part1() {
        assert_eq!(
            find_tag_idx("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap(),
            7
        );
    }
    #[test]
    fn part2_0() {
        assert_eq!(
            find_tag_idx("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
            19
        );
    }
}

// Determine if all characters in the given str are unique
fn is_tag(candidate: &str) -> bool {
    for (i, c) in candidate.char_indices() {
        // For each char search the string for it
        for (j, other_char) in candidate.char_indices() {
            if i == j {
                // Skip the current occurrence
                continue;
            }
            if c == other_char {
                return false;
            }
        }
    }
    true
}

fn find_tag_idx(buffer: &str, tag_len: usize) -> color_eyre::Result<usize> {
    let mut range = tag_len..buffer.len();
    loop {
        if let Some(i) = range.next() {
            let candidate = &buffer[i - tag_len..i];
            if is_tag(candidate) {
                break Some(i);
            }
        } else {
            break None;
        }
    }
    .map_or_else(|| Err(eyre!("No packet found!")), Ok)
}

const PKT_TAG_LEN: usize = 4;
const MSG_TAG_LEN: usize = 14;

fn main() -> color_eyre::Result<()> {
    let signal_buffer = include_str!("input.txt");

    let pkt_idx = find_tag_idx(signal_buffer, PKT_TAG_LEN)?;
    let msg_idx = find_tag_idx(signal_buffer, MSG_TAG_LEN)?;

    println!("PKT: {pkt_idx}, MSG: {msg_idx}");
    Ok(())
}
