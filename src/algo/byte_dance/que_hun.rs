use std::collections::HashMap;

fn count_que_tou(cards: &[u8]) -> u8 {
    let mut i = 0;
    while i < cards.len() - 1 {
        if cards[i] == cards[i + 1] {
            if i + 2 < cards.len() && cards[i] != cards[i + 2] {
                return 1;
            }
            i += 2;
            continue;
        }
        i += 1;
    }
    0
}

fn count_ke_zi(cards: &[u8]) -> u8 {
    let mut i = 0;
    let mut count = 0;
    while i < cards.len() - 2 {
        if cards[i] == cards[i + 1]
            && cards[i] == cards[i + 2]
            && (i + 3 >= cards.len() || cards[i] != cards[i + 3])
        {
            count += 1;
            i += 3;
        } else {
            i += 1;
        }
    }
    count
}

fn count_shun_zi(cards: &[u8]) -> u8 {
    let mut i = 0;
    let mut count = 0;
    while i < cards.len() - 2 {
        if cards[i] == cards[i + 1] - 1
            && cards[i] == cards[i + 2] - 2
            && (i + 3 >= cards.len() || cards[i] != cards[i + 3])
        {
            count += 1;
            i += 3;
        } else {
            i += 1;
        }
    }
    count
}

#[inline]
fn is_winning(card: &[u8]) -> bool {
    count_que_tou(card) == 1 && (count_ke_zi(card) + count_shun_zi(card)) == 4
}

fn is_hu(cards: &mut Vec<u8>) -> bool {
    if cards.is_empty() {
        return true;
    }
    let card0 = cards[0];
    let count0 = cards.iter().filter(|&&c| c == card0).count();
    if cards.len()%3!=0 && count0>=2 {
        cards.remove(0);
        cards.remove(0);
        if is_hu(cards) {
            return true;
        }
        cards.push(card0);
        cards.push(card0);
    }
    false
}

fn requiring_cards(mut cards: Vec<u8>) -> Vec<u8> {
    let mut cards_left = (1u8..=9)
        .zip([4u8; 9].iter().copied())
        .collect::<HashMap<_, _>>();
    let mut ans = vec![];
    for i in cards.iter() {
        *cards_left.get_mut(i).unwrap() -= 1;
    }
    for (left, _) in cards_left.into_iter().filter(|(_, c)| c.ne(&0)) {
        cards.push(left);
        cards.sort_unstable();
        if is_winning(cards.as_slice()) {
            ans.push(left);
        }
        cards.remove(cards.binary_search(&left).unwrap());
    }
    ans.sort_unstable();
    ans
}

#[test]
fn test() {
    let cards = vec![1, 1, 1, 2, 2, 2, 5, 5, 5, 6, 6, 6, 9];
    assert_eq!(requiring_cards(cards), vec![9]);
}
