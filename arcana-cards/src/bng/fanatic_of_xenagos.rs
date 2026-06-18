//! Fanatic of Xenagos — `{1}{R}{G}` 3/3 Centaur Warrior with Trample.
//! Tribute 1 (an ETB choice that isn't in the keyword surface) and its
//! "if tribute wasn't paid" conditional ETB pump are GAP'd — the tribute
//! mechanic and its paid/unpaid state aren't expressible here.

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
    let name = reg.interner_mut().intern("Fanatic of Xenagos");
    let centaur = reg.interner_mut().intern("Centaur");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Tribute 1 — not in the usable keyword surface; the
        // opponent's "may add a +1/+1 counter as it enters" choice is
        // unmodeled.
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            // GAP: intervening-if "if tribute wasn't paid" — tribute
            // paid/unpaid state is not a predicate available here.
            intervening_if: None,
            effect: tribute_unpaid,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn tribute_unpaid(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if tribute wasn't paid, it gets +1/+1 and gains haste" — gated
    // on the unmodeled tribute choice, so the whole effect is omitted
    // rather than firing unconditionally (which would be wrong).
    Vec::new()
}
