//! Blind Hunter — `{2}{W}{B}` 2/2 Bat (W/B).
//!
//! * Flying (keyword).
//! * Haunt — "When this creature dies, exile it haunting target creature."
//!   GAP: Haunt has no keyword variant and no demonstrated primitive to exile
//!   a card "haunting" a creature (the haunt-link mechanic is unmodeled).
//! * "When this creature enters or the creature it haunts dies, target player
//!   loses 2 life and you gain 2 life." The enters half is wired as a targeted
//!   ETB trigger (target player loses 2, you gain 2). The "creature it haunts
//!   dies" half depends on the unmodeled haunt link and is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blind Hunter");
    let bat = reg.interner_mut().intern("Bat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying], // GAP: Haunt is not in the usable keyword surface.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: drain_target_player,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            }),
    )
}

fn drain_target_player(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![
        Effect::LoseLife { player: *p, amount: 2 },
        Effect::GainLife { player: trig.controller, amount: 2 },
    ]
}
