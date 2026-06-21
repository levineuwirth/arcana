//! Burning-Rune Demon — `{4}{B}{B}` 6/6 black Demon Berserker.
//!
//! * Flying.
//! * When this creature enters, you may search your library for exactly two
//!   cards not named Burning-Rune Demon that have different names. If you
//!   do, reveal those cards. An opponent chooses one of them. Put the
//!   chosen card into your hand and the other into your graveyard, then
//!   shuffle.
//!
//! GAP (ETB effect): a two-card tutor with a different-names constraint and
//! an opponent-chosen hand/graveyard split is not expressible from the
//! usable effect catalog (`TutorToHand` searches one card with no
//! opponent-choice split). The ETB trigger shell is retained with a GAP'd
//! effect.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burning-Rune Demon");
    let demon = reg.interner_mut().intern("Demon");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_two_card_tutor_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_two_card_tutor_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: search for exactly two differently-named cards, opponent splits
    // one to hand / one to graveyard — not expressible.
    Vec::new()
}
