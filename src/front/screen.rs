use strum::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Screen {
    Welcome,
    Feed,
    Code,
    Friends,
    Stats,
}

impl Screen {
    const ALL: &'static [Self] = &[Self::Welcome, Self::Feed];

    pub fn next(self) -> Option<Screen> {
        Self::ALL
            .get(
                Self::ALL
                    .iter()
                    .copied()
                    .position(|screen| screen == self)
                    .expect("Screen must exist")
                    + 1,
            )
            .copied()
    }

    pub fn previous(self) -> Option<Screen> {
        let position = Self::ALL
            .iter()
            .copied()
            .position(|screen| screen == self)
            .expect("Screen must exist");

        if position > 0 {
            Some(Self::ALL[position - 1])
        } else {
            None
        }
    }
}
