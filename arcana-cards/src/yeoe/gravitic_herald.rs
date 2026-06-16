//! Gravitic Herald — `{3}{B}` 2/4 black Creature — Human Soldier.
//!
//! Lifelink. When this creature enters, seek a nonland permanent card with mana
//! value 3 or less; that card gains warp {0} until end of turn; you lose 2 life.
//!
//! The ETB trigger has three parts. "Seek a card" (a hidden library search with
//! no reveal, Alchemy-era) has no `Effect` variant, and granting warp {0} to
//! that card is also not expressible (Warp is a pre-wired marker keyword whose
//! rules are deferred). Those two parts are GAP'd; the "you lose 2 life" rider
//! IS expressible and is emitted.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Gravitic Herald");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_seek_warp_loselife,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_seek_warp_loselife(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek a nonland permanent card with mana value 3 or less" (hidden
    // library search, no Effect variant) and "that card gains warp {0} until
    // end of turn" (Warp rules deferred) are not expressible.
    vec![Effect::LoseLife { player: trig.controller, amount: 2 }]
}
