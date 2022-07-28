use crate::common::*;

// payload! {
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct Whitelist {
    pub president: Option<President>,
    pub listers: Vec<Lister>, // 4 + (lister * amount)
}
// }

impl Whitelist {
    pub const MAX_LISTERS_COUNT: usize = 25;
    pub const LEN: usize = (1 + President::LEN) + (4 + Lister::LEN * Whitelist::MAX_LISTERS_COUNT);
    #[inline]
    pub fn set_president(&mut self, president: Option<President>) {
        self.president = president;
    }
    #[inline]
    pub fn majority_num(&self) -> usize {
        self.listers.len() / 2
    }
    /**
     * This method will rearrange the listers
     */
    #[inline]
    pub fn push_keys<K: AsRef<[Pubkey]>>(&mut self, pubkeys: K) {
        self.listers
            .extend(pubkeys.as_ref().into_iter().map(|&key| Lister::new(key)));
        self.listers
            .sort_unstable_by(|a, b| a.pubkey.cmp(&b.pubkey));
        self.listers.dedup();
    }
    #[inline]
    pub fn remove_keys<K: AsRef<[Pubkey]>>(&mut self, pubkeys: K) {
        self.listers
            .retain(|lister| !pubkeys.as_ref().contains(&lister.pubkey));
    }
    #[inline]
    pub fn contains_keys<K: AsRef<[u8]>>(&self, pubkeys: &[K]) -> bool {
        if pubkeys.is_empty() {
            return false;
        }
        let mut i = pubkeys.len();
        for Lister { pubkey, .. } in self.listers.iter() {
            for target_key in pubkeys {
                if pubkey.as_ref() == target_key.as_ref() {
                    i -= 1;
                    if i == 0 {
                        return true;
                    }
                    break;
                }
            }
        }
        false
    }
    #[inline]
    pub fn contains_key<K: AsRef<[u8]>>(&self, pubkey: K) -> bool {
        self.contains_keys(&[pubkey])
    }
    #[inline]
    pub fn find_by_key<K: AsRef<[u8]>>(&self, _pubkey: K) -> Option<&Lister> {
        self.listers
            .iter()
            .find(|Lister { pubkey, .. }| pubkey.as_ref() == _pubkey.as_ref())
    }
    #[inline]
    pub fn find_by_key_mut<K: AsRef<[u8]>>(&mut self, _pubkey: K) -> Option<&mut Lister> {
        self.listers
            .iter_mut()
            .find(|Lister { pubkey, .. }| pubkey.as_ref() == _pubkey.as_ref())
    }
}

impl Display for Whitelist {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Whitelist").field(&self.listers).finish()
    }
}

impl Deref for Whitelist {
    type Target = Vec<Lister>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.listers
    }
}

impl DerefMut for Whitelist {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.listers
    }
}

impl AsSelf for Whitelist {}
